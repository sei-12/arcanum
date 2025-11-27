use crate::{LevelNum, MpNum, StatusNum, passive::list::PassiveList, potential::Potential};

#[derive(Debug, Clone)]
pub struct LtCommon {
    // pub passive: PassiveList,
    passive: PassiveList,
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
    pub fn int(&self) -> StatusNum {
        self.potential.int()
    }

    pub fn dex(&self) -> StatusNum {
        self.potential.dex()
    }

    pub fn str(&self) -> StatusNum {
        self.potential.str()
    }

    pub fn vit(&self) -> StatusNum {
        self.potential.vit()
    }

    pub fn agi(&self) -> StatusNum {
        self.potential.agi()
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
    }

    pub fn physics_attuck(&self) -> StatusNum {
        let base = (self.str() * 3.0 + self.dex()) / 4.0;
        base * self.level_scale()
    }

    pub fn max_hp(&self) -> StatusNum {
        let base = (self.vit() * 6.0 + self.dex()) / 7.0;
        let hp_scale = 3.0;
        let enemy_hp_scale = { if self.is_enemy { 4.0 } else { 1.0 } };

        base * hp_scale * enemy_hp_scale * self.level_scale()
    }

    pub fn hp(&self) -> StatusNum {
        if self.hp > self.max_hp() {
            self.max_hp()
        } else {
            self.hp
        }
    }

    pub fn recv_magic_dmg_mag(&self) -> StatusNum {
        self.passive.status().recv_magic_dmg_mag
    }

    pub fn recv_physics_dmg_mag(&self) -> StatusNum {
        self.passive.status().recv_physics_dmg_mag
    }

    pub fn add_heal_mp(&self) -> MpNum {
        self.passive.status().add_heal_mp
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
