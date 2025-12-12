use crate::{LevelNum, MpNum, StatusNum, passive::PassiveList, potential::Potential, weapon::Weapon};

#[derive(Debug, Clone)]
pub struct LtCommon {
    pub passive: PassiveList,
    potential: Potential,
    level: LevelNum,
    hp: StatusNum,
    weapon: Option<Weapon>,
}

impl LtCommon {
    pub(super) fn new(potential: Potential, level: LevelNum) -> Self {
        let mut tmp = Self {
            potential,
            level,
            hp: 0.0,
            passive: PassiveList::default(),
            weapon: None,
        };

        tmp.hp = tmp.max_hp();

        tmp
    }

    pub(crate) fn new_with_weapon(potential: Potential, level: LevelNum, weapon: Weapon) -> Self {
        let mut tmp = Self {
            potential,
            level,
            hp: 0.0,
            passive: PassiveList::default(),
            weapon: Some(weapon),
        };

        tmp.hp = tmp.max_hp();

        tmp
    }
}

impl LtCommon {
    /// 0.0以上であることが保証されている
    pub fn int(&self) -> StatusNum {
        let tmp = self.potential.int() + self.passive.status().add_int;
        if tmp < 0.0 { 0.0 } else { tmp }
    }

    /// 0.0以上であることが保証されている
    pub fn dex(&self) -> StatusNum {
        let tmp = self.potential.dex() + self.passive.status().add_dex;
        if tmp < 0.0 { 0.0 } else { tmp }
    }

    /// 0.0以上であることが保証されている
    pub fn str(&self) -> StatusNum {
        let tmp = self.potential.str() + self.passive.status().add_str;
        if tmp < 0.0 { 0.0 } else { tmp }
    }

    /// 0.0以上であることが保証されている
    pub fn vit(&self) -> StatusNum {
        let tmp = self.potential.vit() + self.passive.status().add_vit;
        if tmp < 0.0 { 0.0 } else { tmp }
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
            + self.weapon.as_ref().map(|w| w.m_atk).unwrap_or(0.0)
    }

    pub fn physics_attuck(&self) -> StatusNum {
        let base = (self.str() * 3.0 + self.dex()) / 4.0;
        base * self.level_scale()
            * self.passive.status().physics_attuck_mag_buff.get()
            * self.passive.status().physics_attuck_mag_debuff.get()
            + self.weapon.as_ref().map(|w| w.p_atk).unwrap_or(0.0)
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
    
    pub fn mp_heal(&self) -> MpNum {
        todo!()
    } 

    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }
}

impl LtCommon {
    pub(crate) fn accept_damage(&mut self, dmg: StatusNum) {
        // HPは0を下回ることがある
        self.hp -= dmg;
    }

    pub(crate) fn accept_heal(&mut self, heal: StatusNum) {
        self.hp += heal;
        if self.hp > self.max_hp() {
            self.hp = self.max_hp()
        }
    }
}
