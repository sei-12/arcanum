use std::sync::Arc;

use crate::{
    ButtleArgs, CharArg, CooldownNum, EnemyArg, HateNum, LevelNum, MpNum, NUM_MAX_CHAR_IN_TEAM,
    NUM_MAX_ENEMYS_IN_WAVE, SpNum, StaticCharData, StaticEnemyData, StaticPassiveId, StatusNum,
    WinOrLoseOrNextwave,
    damage::{self, Damage},
    living_thing::{ButtleChar, ButtleEnemy, LtCommon, LtId},
    passive::{Passive, PassiveUpdateStateMessage},
    skill::{RuntimeSkillId, StaticSkillData},
};

#[derive(Debug, Clone)]
pub struct GameState {
    chars: Chars<ButtleChar>,
    enemys: Enemys<ButtleEnemy>,
    player_mp: mp::Mp,
}

impl GameState {
    pub(crate) fn new(args: &ButtleArgs) -> Result<Self, crate::Error> {
        let chars = Chars::<ButtleChar>::new(args.chars.as_slice())?;
        let enemys = Enemys::<ButtleEnemy>::new(args.enemys.clone())?;
        Ok(Self {
            chars,
            enemys,
            player_mp: mp::Mp::default(),
        })
    }

    pub(crate) fn update(&mut self, message: &UpdateStateMessage) -> Option<WinOrLoseOrNextwave> {
        match message {
            UpdateStateMessage::ConsumeMp(mp) => {
                self.player_mp.consume(*mp);
            }
            UpdateStateMessage::AddHate(char_id, hate) => {
                self.chars.get_mut(*char_id).add_hate(*hate);
            }
            UpdateStateMessage::ConsumeSp(enemy_id, num) => {
                self.enemys.get_mut(*enemy_id).consume_sp(*num);
            }
            UpdateStateMessage::Damage(dmg) => {
                // 複雑ではなかったけど処理が冗長だったので関数に切り出した
                if let Some(res) = self.accept_damage(dmg) {
                    return Some(res);
                };
            }
            UpdateStateMessage::HealHp(lt_id, num) => {
                self.get_lt_mut(*lt_id).accept_heal(*num);
            }
            UpdateStateMessage::HealMp(num) => {
                self.player_mp.heal(*num);
            }
            UpdateStateMessage::HealSkillCooldown(char_id, skill_id, num) => {
                self.chars
                    .get_mut(*char_id)
                    .skills
                    .heal_skill_cooldown(*skill_id, *num);
            }
            UpdateStateMessage::HealSkillCooldownAll(char_id, num) => {
                self.chars
                    .get_mut(*char_id)
                    .skills
                    .heal_skill_cooldown_all(*num);
            }
            UpdateStateMessage::HealSp(enemy_id, num) => {
                self.enemys.get_mut(*enemy_id).heal_sp(*num);
            }
            UpdateStateMessage::SetSkillCooldown(char_id, skill_id, num) => {
                self.chars
                    .get_mut(*char_id)
                    .skills
                    .set_cooldown(*skill_id, *num);
            }
            UpdateStateMessage::UpdatePassiveState(lt_id, passive_id, msg) => {
                self.get_lt_mut(*lt_id)
                    .passive
                    .update_state(*passive_id, msg);
            }
            UpdateStateMessage::AddPassive(lt_id, passive) => {
                // cloneしない方法が思い浮かばない
                // 初期化ずみのデータを渡すのではなく、初期化するための情報を渡すとかに変えるくらい
                // 面倒なのでやらない
                self.get_lt_mut(*lt_id).passive.add(passive.clone());
            }
        }

        None
    }

    fn accept_damage(&mut self, dmg: &Damage) -> Option<WinOrLoseOrNextwave> {
        let target = self.get_lt_mut(dmg.target());
        target.accept_damage(dmg.dmg());

        if target.is_dead() {
            if dmg.target().is_char() {
                return Some(WinOrLoseOrNextwave::Lose);
            }

            if self.current_wave_enemys_all_dead() {
                if self.current_wave_is_last_wave() {
                    return Some(WinOrLoseOrNextwave::Win);
                } else {
                    return Some(WinOrLoseOrNextwave::Nextwave);
                }
            }
        }

        None
    }

    pub(crate) fn get_lt_mut(&mut self, id: LtId) -> &mut LtCommon {
        match id {
            LtId::Char(char_id) => self.chars.get_mut(char_id).lt_mut(),
            LtId::Enemy(enemy_id) => self.enemys.get_mut(enemy_id).lt_mut(),
        }
    }
}

//--------------------------------------------------//
//                                                  //
//                PUBLIC GAME STATE                 //
//                                                  //
//--------------------------------------------------//

impl GameState {
    pub fn player_mp(&self) -> MpNum {
        self.player_mp.get()
    }

    pub fn get_char(&self, id: RuntimeCharId) -> &ButtleChar {
        self.chars.get(id)
    }

    pub fn get_enemy(&self, id: RuntimeEnemyId) -> &ButtleEnemy {
        self.enemys.get(id)
    }

    pub fn get_lt(&self, id: LtId) -> &LtCommon {
        match id {
            LtId::Char(char_id) => self.chars.get(char_id).lt(),
            LtId::Enemy(enemy_id) => self.enemys.get(enemy_id).lt(),
        }
    }

    pub fn chars(&self) -> std::slice::Iter<'_, ButtleChar> {
        assert!(
            !self.chars.inner.is_empty(),
            "キャラクターが1人以上いることは保証されているはず"
        );
        self.chars.inner.iter()
    }

    pub fn enemys(&self) -> impl Iterator<Item = &ButtleEnemy> {
        self.enemys.current_wave.iter()
    }

    pub fn living_enemys(&self) -> impl Iterator<Item = &ButtleEnemy> {
        self.enemys
            .current_wave
            .iter()
            .filter(|item| !item.lt().is_dead())
    }

    pub fn get_max_hate_char(&self) -> &ButtleChar {
        // max_by_key関数は同値がある場合に最後の要素を返す。
        // そのためのrev()
        self.chars().rev().max_by_key(|c| c.hate()).unwrap()
    }

    pub fn current_wave_enemys_all_dead(&self) -> bool {
        self.enemys
            .current_wave
            .iter()
            .all(|enemy| enemy.lt().is_dead())
    }

    pub fn current_wave_is_last_wave(&self) -> bool {
        self.enemys.current_wave_is_last_wave()
    }

    pub fn chars_with_living_check(&self) -> CharIterWithLivingCheck {
        CharIterWithLivingCheck::new()
    }

    pub fn enemys_with_living_check(&self) -> EnemyIterWithLivingCheck {
        EnemyIterWithLivingCheck::new()
    }
}

pub struct EnemyIterWithLivingCheck {
    idx: usize,
}

impl EnemyIterWithLivingCheck {
    pub(super) fn new() -> Self {
        Self { idx: 0 }
    }

    pub fn next_livint_enemy<'a>(&mut self, state: &'a GameState) -> Option<&'a ButtleEnemy> {
        loop {
            let enemy = state.enemys.get_by_idx_from_current_wave(self.idx)?;
            self.idx += 1;
            if !enemy.lt().is_dead() {
                break Some(enemy);
            }
        }
    }
}

pub struct CharIterWithLivingCheck {
    idx: usize,
}
impl CharIterWithLivingCheck {
    pub(super) fn new() -> Self {
        Self { idx: 0 }
    }

    pub fn next_livint_char<'a>(&mut self, state: &'a GameState) -> Option<&'a ButtleChar> {
        loop {
            let char = state.chars.get_by_idx(self.idx)?;
            self.idx += 1;
            if !char.lt().is_dead() {
                break Some(char);
            }
        }
    }
}

//--------------------------------------------------//
//                                                  //
//               UPDATE STATE MESSAGE               //
//                                                  //
//--------------------------------------------------//
#[derive(Debug, Clone)]
pub(crate) enum UpdateStateMessage {
    Damage(damage::Damage),
    HealHp(LtId, StatusNum),
    ConsumeMp(MpNum),
    HealMp(MpNum),
    ConsumeSp(RuntimeEnemyId, SpNum),
    HealSp(RuntimeEnemyId, SpNum),
    UpdatePassiveState(LtId, StaticPassiveId, PassiveUpdateStateMessage),
    AddPassive(LtId, Box<dyn Passive>),
    SetSkillCooldown(RuntimeCharId, RuntimeSkillId, CooldownNum),
    HealSkillCooldown(RuntimeCharId, RuntimeSkillId, CooldownNum),
    HealSkillCooldownAll(RuntimeCharId, CooldownNum),
    AddHate(RuntimeCharId, HateNum),
}

//--------------------------------------------------//
//                                                  //
//                      CHARS                       //
//                                                  //
//--------------------------------------------------//

pub(crate) trait ButtleCharsItem: Sized {
    fn new(
        data: &'static crate::StaticCharData,
        level: LevelNum,
        id: RuntimeCharId,
        skills: &[&'static StaticSkillData],
    ) -> Result<Self, crate::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RuntimeCharId(u8);

#[derive(Debug, Clone)]
struct Chars<T: ButtleCharsItem> {
    inner: Vec<T>,
}

impl<T: ButtleCharsItem> Chars<T> {
    fn new(char_datas: &[CharArg]) -> Result<Self, crate::Error> {
        if char_datas.is_empty() || char_datas.len() > NUM_MAX_CHAR_IN_TEAM as usize {
            return Err(crate::Error::InvalidNumTeamMembers {
                got_num_members: char_datas.len(),
            });
        };

        let mut chars = Vec::new();
        for (i, c) in char_datas.iter().enumerate() {
            let id = RuntimeCharId(i as u8);
            let t = T::new(c.static_data, c.level, id, &c.skills)?;
            chars.push(t);
        }

        assert_eq!(chars.len(), char_datas.len());

        Ok(Self { inner: chars })
    }

    fn get(&self, id: RuntimeCharId) -> &T {
        &self.inner[id.0 as usize]
    }

    fn get_mut(&mut self, id: RuntimeCharId) -> &mut T {
        &mut self.inner[id.0 as usize]
    }

    fn get_by_idx(&self, idx: usize) -> Option<&T> {
        self.inner.get(idx)
    }
}

//--------------------------------------------------//
//                                                  //
//                      ENEMYS                      //
//                                                  //
//--------------------------------------------------//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeEnemyId {
    wave_idx: u8,
    idx: u8,
}

pub(crate) trait ButtleEnemysItem {
    fn new(data: &'static StaticEnemyData, level: LevelNum, id: RuntimeEnemyId) -> Self;
    fn runtime_id(&self) -> RuntimeEnemyId;
}

#[derive(Debug, Clone)]
struct Enemys<T: ButtleEnemysItem> {
    // 不変であり、コピーのコストが高いと判断した
    data: Arc<Vec<Vec<EnemyArg>>>,
    current_wave: Vec<T>,
    current_wave_idx: usize,
}

impl<T: ButtleEnemysItem> Enemys<T> {
    pub fn new(data: Arc<Vec<Vec<EnemyArg>>>) -> Result<Self, crate::Error> {
        if data.is_empty() {
            return Err(crate::Error::WavesIsEmpty);
        }

        if data
            .iter()
            .any(|wave| wave.is_empty() || wave.len() > NUM_MAX_ENEMYS_IN_WAVE)
        {
            return Err(crate::Error::InvalidNumEnemysInWave(data));
        }

        let current_wave = data
            .first()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, enemy)| {
                T::new(
                    enemy.static_data,
                    enemy.level,
                    RuntimeEnemyId {
                        wave_idx: 0,
                        idx: i as u8,
                    },
                )
            })
            .collect::<Vec<_>>();

        Ok(Self {
            data,
            current_wave,
            current_wave_idx: 0,
        })
    }

    fn current_wave_is_last_wave(&self) -> bool {
        assert!(!self.data.is_empty());
        self.current_wave_idx == self.data.len() - 1
    }

    fn get_by_idx_from_current_wave(&self, idx: usize) -> Option<&T> {
        self.current_wave.get(idx)
    }

    fn get_mut(&mut self, id: RuntimeEnemyId) -> &mut T {
        assert_eq!(self.current_wave_idx, id.wave_idx as usize);
        &mut self.current_wave[id.idx as usize]
    }

    fn get(&self, id: RuntimeEnemyId) -> &T {
        assert_eq!(self.current_wave_idx, id.wave_idx as usize);
        &self.current_wave[id.idx as usize]
    }
}

//--------------------------------------------------//
//                                                  //
//                        MP                        //
//                                                  //
//--------------------------------------------------//

mod mp {
    use crate::MpNum;

    #[derive(Debug, Clone, Default)]
    pub(super) struct Mp {
        value: MpNum,
    }

    impl Mp {
        pub fn consume(&mut self, num: MpNum) {
            if num > self.value {
                self.value = 0
            } else {
                self.value -= num
            }
        }

        pub fn heal(&mut self, num: MpNum) {
            self.value += num;
        }

        pub fn get(&self) -> MpNum {
            self.value
        }
    }
}
