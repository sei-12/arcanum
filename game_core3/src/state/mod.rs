use crate::{
    MpNum,
    args::{ContainerArgs, EnemyData},
    buttle_enemy::ButtleEnemy,
    enemys::{ButtleEnemys, RuntimeEnemyId},
    event::Event,
    lt_common::LtCommon,
    state::chars::{ButtleChars, RuntimeCharId},
};

pub mod chars;
mod mp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LtId {
    Enemy(RuntimeEnemyId),
    Char(RuntimeCharId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Player,
    Enemy,
}

#[derive(Debug, Clone)]
pub struct GameState {
    focused_enemy: Option<RuntimeEnemyId>,
    chars: ButtleChars,
    enemys: ButtleEnemys<ButtleEnemy, EnemyData>,
    player_mp: mp::Mp,
    enemy_mp: mp::Mp,
}

impl GameState {
    pub(crate) fn new(arg: &ContainerArgs) -> Result<Self, crate::Error> {
        Ok(Self {
            focused_enemy: None,
            chars: ButtleChars::new(&arg.chars)?,
            enemys: ButtleEnemys::new(&arg.enemy),
            player_mp: mp::Mp::default(),
            enemy_mp: mp::Mp::default(),
        })
    }
    // Cowでeventを渡してもいいかも
    pub fn accept_event(&mut self, event: Event) {
        match event {
            Event::AddHate { char_id, hate } => {
                self.chars.get_mut_char(char_id).add_hate(hate);
            }
            Event::AddPassive { target_id, passive } => {
                let target = self.get_lt_mut(target_id);
                target.passive.add(passive).unwrap();
            }
            Event::ChangeFocusEnemy { enemy_id } => self.focused_enemy = Some(enemy_id),
            Event::ConsumeMp { side, mp } => match side {
                Side::Enemy => self.enemy_mp.consume(mp),
                Side::Player => self.player_mp.consume(mp),
            },
            Event::Damage(dmg) => {
                let target = self.get_lt_mut(dmg.target());
                target.accept_damage(dmg.dmg());
            }
            Event::DeadEnemy { enemy_id: _ } => {}
            Event::GameEnd(_) => {}
            Event::GoNextWave => {
                self.enemys.go_next_wave();
            }
            Event::HealMp { side, mp } => match side {
                Side::Enemy => self.enemy_mp.heal(mp),
                Side::Player => self.player_mp.heal(mp),
            },
            Event::HealSkillCooldown {
                char_id,
                skill_id,
                heal_num,
            } => {
                let char = self.chars.get_mut_char(char_id);
                char.skills.heal_skill_cooldown(skill_id, heal_num).unwrap();
            }
            Event::Log(_) => {}
            Event::SetSkillCooldown {
                char_id,
                skill_id,
                cooldown,
            } => {
                self.chars
                    .get_mut_char(char_id)
                    .skills
                    .set_cooldown(skill_id, cooldown)
                    .unwrap();
            }
            Event::TurnStart(_) => {}
            Event::UpdatePassiveState {
                target_id,
                passive_id,
                msg,
            } => {
                let target = self.get_lt_mut(target_id);
                target.passive.update_state(passive_id, &msg).unwrap();
            }
        }
    }

    pub fn chars(&self) -> &ButtleChars {
        &self.chars
    }

    pub fn enemys(&self) -> &ButtleEnemys<ButtleEnemy, EnemyData> {
        &self.enemys
    }

    pub fn focused_enemy(&self) -> Option<&ButtleEnemy> {
        self.focused_enemy.map(|id| self.enemys.get(id))
    }

    /// # Panic
    /// すでにゲームが終了している場合、パニックになる
    pub(crate) fn get_enemy_with_highest_target_priority(&self) -> &ButtleEnemy {
        assert!(!self.check_game_end().game_ended());
        self.enemys_sorted_by_target_priority()
            .next()
            .expect("ゲームが終了していない場合、必ず1体以上の敵がいる")
    }

    pub fn enemys_sorted_by_target_priority(&self) -> impl Iterator<Item = &ButtleEnemy> {
        // フォーカスされた敵が先頭でそれ以外は配列の順番通り
        let f = self.focused_enemy();

        // Optionをinto_iterするとSomeの場合は要素が追加され、Noneの場合は何も追加されない
        f.into_iter()
            .chain(self.enemys.current_wave_enemys().filter(move |enemy| {
                let is_not_focused_enemy = Some(enemy.runtime_id()) != f.map(|f| f.runtime_id());
                !enemy.lt().is_dead() && is_not_focused_enemy
            }))
    }

    pub fn get_lt(&self, lt_id: LtId) -> &LtCommon {
        match lt_id {
            LtId::Char(id) => self.chars().get_char(id).lt(),
            LtId::Enemy(id) => self.enemys.get(id).lt(),
        }
    }

    fn get_lt_mut(&mut self, lt_id: LtId) -> &mut LtCommon {
        match lt_id {
            LtId::Char(id) => self.chars.get_mut_char(id).lt_mut(),
            LtId::Enemy(id) => self.enemys.get_mut(id).lt_mut(),
        }
    }

    pub fn check_game_end(&self) -> CheckGameEndResult {
        if self.chars.chars().iter().any(|char| char.lt().is_dead()) {
            return CheckGameEndResult::Lose;
        }
        if self.enemys.current_wave_all_dead() {
            if self.enemys.current_wave_is_last_wave() {
                CheckGameEndResult::Win
            } else {
                CheckGameEndResult::GoNextWave
            }
        } else {
            CheckGameEndResult::None
        }
    }

    pub fn player_mp(&self) -> MpNum {
        self.player_mp.get()
    }

    pub fn enemy_mp(&self) -> MpNum {
        self.enemy_mp.get()
    }
}

pub enum CheckGameEndResult {
    Win,
    GoNextWave,
    Lose,
    None,
}

impl CheckGameEndResult {
    pub(crate) fn game_ended(&self) -> bool {
        matches!(self, CheckGameEndResult::Lose | CheckGameEndResult::Win)
    }
}
