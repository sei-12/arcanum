use crate::{
    container::Container,
    damage::{DamageType, calc_damage},
    error::GameError,
    skills::{StaticActiveSkill, dmg_msg_template},
};

pub const SKILL: StaticActiveSkill = StaticActiveSkill {
    id: 4,
    name: "ウィンドスラッシュ",
    need_mp: 60.0,
    call,
    text: TEXT,
};

const TEXT: &str = "消費MP 60
クールタイム 300
ヘイト値 80
敵にスキルダメージ1.0の物理ダメージを与える。
スキル使用者のSTRが10以上ならスキルダメージを0.1加算する。
スキル使用者のAGIが11以上ならこのスキルのクールタイムは100小さくなる。
";

fn call(static_user_id: usize, con: &mut Container) -> Result<(), GameError> {
    let user = con.get_char(static_user_id)?;
    let user_name = user.static_data.name;
    let enemy = con.get_enemy();
    let enemy_name = enemy.static_data.name;
    let mut skill_atk = 1.0;
    if user.str() >= 10.0 {
        skill_atk += 0.1;
    }
    let dmg = calc_damage(user, enemy, DamageType::Physics, skill_atk);

    let mut cooltime = 300.0;
    if user.agi() >= 12.0 {
        cooltime -= 100.0;
    }

    con.update_char(static_user_id, |user| {
        user.set_skill_cooltime(SKILL.id, cooltime)?;
        user.add_hate(80.0);
        Ok(())
    })?;

    con.update_enemy(|enemy| {
        enemy.accept_damage(dmg);
    });

    con.consume_player_side_mp(SKILL.need_mp);

    con.log(dmg_msg_template(
        user_name,
        SKILL.name,
        enemy_name,
        dmg,
        DamageType::Physics,
    ));

    Ok(())
}
