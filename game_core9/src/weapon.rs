use crate::StatusNum;

#[derive(Debug, Clone, Copy)]
pub enum WeaponType {
    Sword,
    MagicBook,
    Cane,
    Spear,
    Hammer,
    SwordAndShield,
    SpearAndShield,
    Bow,
}

impl WeaponType {
    pub fn is_sword(&self) -> bool {
        matches!(self, Self::Sword | Self::SwordAndShield)
    }

    pub fn is_spear(&self) -> bool {
        matches!(self, Self::Spear | Self::SpearAndShield)
    }

    pub fn is_shield(&self) -> bool {
        matches!(self, Self::SpearAndShield | Self::SwordAndShield)
    }
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub ty: WeaponType,
    pub p_atk: StatusNum,
    pub m_atk: StatusNum,
}
