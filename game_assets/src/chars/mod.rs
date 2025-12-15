use game_core7::{
    buttle_char::ButtleCharArgs,
    potential::Potential,
    weapon::{Weapon, WeaponType},
};

pub fn elena() -> ButtleCharArgs {
    ButtleCharArgs {
        static_id: 1,
        name: "エレナ",
        level: 1,
        potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
        default_passives: vec![],
        skills: vec![],
        weapon: Weapon {
            m_atk: 10.0,
            p_atk: 10.0,
            ty: WeaponType::Cane,
        },
    }
}

pub fn asya() -> ButtleCharArgs {
    ButtleCharArgs {
        static_id: 2,
        name: "アーシャ",
        level: 1,
        potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
        default_passives: vec![],
        skills: vec![],
        weapon: Weapon {
            m_atk: 10.0,
            p_atk: 10.0,
            ty: WeaponType::Bow,
        },
    }
}

pub fn yuuko() -> ButtleCharArgs {
    ButtleCharArgs {
        static_id: 3,
        name: "幽狐",
        level: 1,
        potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
        default_passives: vec![],
        skills: vec![],
        weapon: Weapon {
            m_atk: 10.0,
            p_atk: 10.0,
            ty: WeaponType::Spear,
        },
    }
}

pub fn kazu() -> ButtleCharArgs {
    ButtleCharArgs {
        static_id: 4,
        name: "カズ",
        level: 1,
        potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
        default_passives: vec![],
        skills: vec![],
        weapon: Weapon {
            m_atk: 10.0,
            p_atk: 10.0,
            ty: WeaponType::SwordAndShield,
        },
    }
}
