use crate::{Num, lt::LtCommon};

pub enum DamageType {
    Physics,
    Magic,
}

pub fn calc_damage(
    attucker: &LtCommon,
    defender: &LtCommon,
    dmg_type: DamageType,
    skill_atk: Num,
) -> Num {
    let atk = match dmg_type {
        DamageType::Magic => attucker.magic_attuck(),
        DamageType::Physics => attucker.physics_attuck(),
    };

    let def = match dmg_type {
        DamageType::Magic => defender.magic_defence(),
        DamageType::Physics => defender.physics_defence(),
    };

    let dmg = atk * skill_atk * def;

    assert!(
        dmg >= 0.0,
        "ダメージは0.0以上である必要があります dmg={}",
        dmg
    );

    dmg
}
