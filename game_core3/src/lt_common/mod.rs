use crate::{LevelNum, StatusNum, passive::list::PassiveList, potential::Potential};

#[derive(Debug, Clone)]
pub struct LtCommon {
    // pub passive: PassiveList,
    pub passive: PassiveList,
    potential: &'static Potential,
    level: LevelNum,
    hp: StatusNum,
    is_enemy: bool,
}

impl LtCommon {
    pub(super) fn new(potential: &'static Potential, level: LevelNum, is_enemy: bool) -> Self {
        let mut tmp = Self {
            potential,
            level,
            hp: 0.0,
            is_enemy,
            passive: PassiveList::default(),
        };

        tmp.hp = tmp.max_hp();

        tmp
    }
}

//--------------------------------------------------//
//                                                  //
//                    POTENTIAL                     //
//                                                  //
//--------------------------------------------------//
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

//--------------------------------------------------//
//                                                  //
//                      STATUS                      //
//                                                  //
//--------------------------------------------------//
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
        let enemy_hp_scale = { if self.is_enemy { 4.0 } else { 1.0 } };

        base * hp_scale
            * enemy_hp_scale
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

//--------------------------------------------------//
//                                                  //
//                       MUT                        //
//                                                  //
//--------------------------------------------------//

impl LtCommon {
    pub(crate) fn accept_damage(&mut self, dmg: StatusNum) {
        self.hp -= dmg;
        if self.hp < 0.0 {
            self.hp = 0.0;
        }
    }
}
