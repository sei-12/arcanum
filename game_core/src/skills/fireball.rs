use crate::{
    container::Container,
    damage::{DamageType, calc_damage},
    error::GameError,
    passive::public_passive,
    skills::{StaticActiveSkill, dmg_msg_template},
};

pub const SKILL: StaticActiveSkill = StaticActiveSkill {
    id: 0,
    name: "ファイヤーボール",
    need_mp: 10.0,
    call,
};

fn call(static_user_id: usize, con: &mut Container) -> Result<(), GameError> {
    let user = con.get_char(static_user_id)?;
    let user_name = user.static_data.name;
    let enemy = con.get_enemy();
    let enemy_name = enemy.static_data.name;
    let skill_atk = 1.1;
    let dmg = calc_damage(user, enemy, DamageType::Magic, skill_atk);

    let mut user = con.get_mut_char(static_user_id)?;
    // 使用者のDEXが4以下なら使用者に2ターンの火傷を付与する
    if user.potential().dex <= 4.0 {
        let burn = public_passive::Burn::new(2);
        user.passive.add(Box::new(burn));
    }

    user.set_skill_cooltime(SKILL.id, 3)?;

    drop(user);

    con.get_mut_enemy().accept_damage(dmg);
    con.consume_player_side_mp(SKILL.need_mp);

    con.log(dmg_msg_template(
        user_name,
        SKILL.name,
        enemy_name,
        dmg,
        DamageType::Magic,
    ));

    Ok(())
}
