use crate::{
    CooldownNum, HateNum, LevelNum, SKILL_COOLDOWN_HEAL_BASE, SpNum, StaticCharData,
    StaticEnemyData, StatusNum,
    passive::PassiveList,
    skill::{ButtleSkills, StaticSkillData},
    state::{ButtleCharsItem, ButtleEnemysItem, RuntimeCharId, RuntimeEnemyId},
};

//--------------------------------------------------//
//                                                  //
//                    POTENTIAL                     //
//                                                  //
//--------------------------------------------------//
#[derive(Debug, Clone)]
pub struct Potential {
    int: StatusNum,
    vit: StatusNum,
    str: StatusNum,
    dex: StatusNum,
    agi: StatusNum,
}

impl Potential {
    pub const fn new(
        int: StatusNum,
        vit: StatusNum,
        str: StatusNum,
        dex: StatusNum,
        agi: StatusNum,
    ) -> Self {
        let sum = agi + dex + int + str + vit;

        if int <= 0.0 {
            panic!("ポテンシャルの値は0より大きい値である必要がある")
        }

        if vit <= 0.0 {
            panic!("ポテンシャルの値は0より大きい値である必要がある")
        }

        if str <= 0.0 {
            panic!("ポテンシャルの値は0より大きい値である必要がある")
        }

        if dex <= 0.0 {
            panic!("ポテンシャルの値は0より大きい値である必要がある")
        }

        if agi <= 0.0 {
            panic!("ポテンシャルの値は0より大きい値である必要がある")
        }

        if sum != 50.0 {
            panic!("ポテンシャルの合計値はちょうど50である必要がある")
        };

        Self {
            int,
            vit,
            str,
            dex,
            agi,
        }
    }

    /// 0より大きいことが保証されている
    pub fn int(&self) -> StatusNum {
        self.int
    }

    /// 0より大きいことが保証されている
    pub fn vit(&self) -> StatusNum {
        self.vit
    }

    /// 0より大きいことが保証されている
    pub fn str(&self) -> StatusNum {
        self.str
    }

    /// 0より大きいことが保証されている
    pub fn dex(&self) -> StatusNum {
        self.dex
    }

    /// 0より大きいことが保証されている
    pub fn agi(&self) -> StatusNum {
        self.agi
    }
}

//--------------------------------------------------//
//                                                  //
//                    LT COMMON                     //
//                                                  //
//--------------------------------------------------//
#[derive(Debug, Clone)]
pub struct LtCommon {
    pub passive: PassiveList,
    potential: &'static Potential,
    level: LevelNum,
    hp: StatusNum,
    name: String,
}

impl LtCommon {
    pub(super) fn new(potential: &'static Potential, level: LevelNum, name: String) -> Self {
        let mut tmp = Self {
            potential,
            level,
            hp: 0.0,
            name,
            passive: PassiveList::default(),
        };

        tmp.hp = tmp.max_hp();

        tmp
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl LtCommon {
    /// 0.0以上であることが保証されている
    pub fn int(&self) -> StatusNum {
        self.potential.int()
    }

    /// 0.0以上であることが保証されている
    pub fn dex(&self) -> StatusNum {
        self.potential.dex()
    }

    /// 0.0以上であることが保証されている
    pub fn str(&self) -> StatusNum {
        let tmp = self.potential.str() + self.passive.status().add_str;
        if tmp < 0.0 { 0.0 } else { tmp }
    }

    /// 0.0以上であることが保証されている
    pub fn vit(&self) -> StatusNum {
        self.potential.vit()
    }

    /// 0.0以上であることが保証されている
    pub fn agi(&self) -> StatusNum {
        let tmp = self.potential.agi() + self.passive.status().add_agi;
        if tmp < 0.0 { 0.0 } else { tmp }
    }
}

impl LtCommon {
    fn level_scale(&self) -> StatusNum {
        (self.level + 10) as f32
    }

    pub fn level(&self) -> LevelNum {
        self.level
    }

    pub fn magic_attuck(&self) -> StatusNum {
        let base = (self.int() * 3.0 + self.dex()) / 4.0;
        base * self.level_scale()
            * self.passive.status().magic_attuck_mag_buff.get()
            * self.passive.status().magic_attuck_mag_debuff.get()
    }

    pub fn physics_attuck(&self) -> StatusNum {
        let base = (self.str() * 3.0 + self.dex()) / 4.0;
        base * self.level_scale()
            * self.passive.status().physics_attuck_mag_buff.get()
            * self.passive.status().physics_attuck_mag_debuff.get()
    }

    pub fn max_hp(&self) -> StatusNum {
        let base = (self.vit() * 6.0 + self.dex()) / 7.0;
        let hp_scale = 3.0;

        base * hp_scale
            * self.level_scale()
            * self.passive.status().max_hp_mag_buff.get()
            * self.passive.status().max_hp_mag_debuff.get()
    }

    pub fn hp(&self) -> StatusNum {
        if self.hp > self.max_hp() {
            self.max_hp()
        } else {
            self.hp
        }
    }

    pub fn recv_magic_dmg_mag(&self) -> StatusNum {
        self.passive.status().recv_magic_dmg_mag.get()
    }

    pub fn recv_physics_dmg_mag(&self) -> StatusNum {
        self.passive.status().recv_physics_dmg_mag.get()
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }
}

impl LtCommon {
    pub(crate) fn accept_damage(&mut self, dmg: StatusNum) {
        self.hp -= dmg;
        if self.hp < 0.0 {
            self.hp = 0.0;
        }
    }

    pub(crate) fn accept_heal(&mut self, heal: StatusNum) {
        self.hp += heal;
        if self.hp > self.max_hp() {
            self.hp = self.max_hp()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LtId {
    Char(RuntimeCharId),
    Enemy(RuntimeEnemyId),
}
impl LtId {
    pub fn is_char(&self) -> bool {
        matches!(self, Self::Char(_))
    }
    pub fn is_enemy(&self) -> bool {
        matches!(self, Self::Enemy(_))
    }
}

//--------------------------------------------------//
//                                                  //
//                   BUTTLE CHAR                    //
//                                                  //
//--------------------------------------------------//

#[derive(Debug, Clone)]
pub struct ButtleChar {
    static_data: &'static StaticCharData,
    lt_common: LtCommon,
    pub skills: ButtleSkills,
    runtime_id: RuntimeCharId,
    hate: HateNum,
}

impl ButtleCharsItem for ButtleChar {
    fn new(
        data: &'static crate::StaticCharData,
        level: LevelNum,
        id: RuntimeCharId,
        skills: &[&'static StaticSkillData],
    ) -> Result<Self, crate::Error> {
        let lt_common = LtCommon::new(&data.potential, level, data.name.to_string());
        let skills = ButtleSkills::new(id, skills)?;
        Ok(Self {
            static_data: data,
            lt_common,
            runtime_id: id,
            skills,
            hate: 0,
        })
    }
}

impl ButtleChar {
    pub(crate) fn runtime_id(&self) -> RuntimeCharId {
        self.runtime_id
    }

    pub fn static_data(&self) -> &'static StaticCharData {
        self.static_data
    }

    pub fn add_hate(&mut self, num: HateNum) {
        self.hate += num;
    }

    pub fn hate(&self) -> HateNum {
        self.hate
    }

    pub fn lt(&self) -> &LtCommon {
        &self.lt_common
    }

    pub fn lt_mut(&mut self) -> &mut LtCommon {
        &mut self.lt_common
    }

    pub fn lt_id(&self) -> LtId {
        LtId::Char(self.runtime_id)
    }

    pub fn cooldown_heal(&self) -> CooldownNum {
        let agi = self.lt().agi() as CooldownNum;
        SKILL_COOLDOWN_HEAL_BASE + agi * 5
    }
}

//--------------------------------------------------//
//                                                  //
//                   BUTTLE ENEMY                   //
//                                                  //
//--------------------------------------------------//

#[derive(Debug, Clone)]
pub struct ButtleEnemy {
    sp: SpNum,
    lt_common: LtCommon,
    static_data: &'static StaticEnemyData,
    runtime_id: RuntimeEnemyId,
}

impl ButtleEnemy {
    pub fn lt(&self) -> &LtCommon {
        &self.lt_common
    }

    pub(crate) fn lt_mut(&mut self) -> &mut LtCommon {
        &mut self.lt_common
    }

    pub fn heal_sp(&mut self, num: SpNum) {
        self.sp += num;
    }
    pub fn consume_sp(&mut self, num: SpNum) {
        if self.sp > num {
            self.sp -= num;
        } else {
            self.sp = 0;
        }
    }

    pub fn sp(&self) -> SpNum {
        self.sp
    }

    pub fn static_data(&self) -> &StaticEnemyData {
        self.static_data
    }

    pub fn lt_id(&self) -> LtId {
        LtId::Enemy(self.runtime_id)
    }
}

impl ButtleEnemysItem for ButtleEnemy {
    fn new(
        data: &'static StaticEnemyData,
        level: LevelNum,
        id: crate::state::RuntimeEnemyId,
    ) -> Self {
        Self {
            sp: 0,
            lt_common: LtCommon::new(&data.potential, level, data.name.to_string()),
            static_data: data,
            runtime_id: id,
        }
    }

    fn runtime_id(&self) -> crate::state::RuntimeEnemyId {
        self.runtime_id
    }
}
