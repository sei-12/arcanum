use crate::{
    container::Container,
    damage::{DamageType, calc_damage},
    error::GameError,
    passive::PassiveIdentifier,
    skills::{StaticActiveSkill, dmg_msg_template},
};

pub const SKILL: StaticActiveSkill = StaticActiveSkill {
    id: 2,
    name: "火炎の大竜巻",
    need_mp: 200.0,
    call,
    text: TEXT,
};

const TEXT: &str = "消費MP 200
クールタイム 700
ヘイト値 230
敵にスキルダメージ2.5の魔法ダメージを与える。
スキル使用者のINTが16以上ならスキルダメージを1.0加算する。
敵が火傷状態ならスキルダメージを0.3加算する。
スキル使用者のAGIが14以上ならヘイト値が40減少する。
";

fn call(static_user_id: usize, con: &mut Container) -> Result<(), GameError> {
    let user = con.get_char(static_user_id)?;
    let user_name = user.static_data.name;
    let enemy = con.get_enemy();
    let enemy_name = enemy.static_data.name;
    let mut skill_atk = 2.5;

    if user.int() >= 16.0 {
        skill_atk += 1.0;
    };

    if enemy.passive.have(PassiveIdentifier::Burn) {
        skill_atk += 0.3;
    }

    let mut hate = 230.0;
    if user.agi() >= 14.0 {
        hate -= 40.0;
    }

    let dmg = calc_damage(user, enemy, DamageType::Magic, skill_atk);

    let cooltime = 700.0;

    con.update_char(static_user_id, |char| {
        char.set_skill_cooltime(SKILL.id, cooltime)?;
        char.add_hate(hate);
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
        DamageType::Magic,
    ));

    Ok(())
}
