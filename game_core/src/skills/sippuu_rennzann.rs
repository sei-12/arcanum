use crate::{
    container::Container,
    damage::{DamageType, calc_damage},
    error::GameError,
    skills::{StaticActiveSkill, dmgs_msg_template},
};

pub const SKILL: StaticActiveSkill = StaticActiveSkill {
    id: 6,
    name: "疾風連斬",
    need_mp: 80.0,
    call,
    text: TEXT,
};

const TEXT: &str = "消費MP 80
クールタイム 350
ヘイト値 60
スキルダメージ0.1の物理ダメージを5回敵に与える。その後、全ての味方のスキルクールタイムをスキル使用者のスキルクールタイム回復力分だけ回復する。
スキル使用者のAGIが14以上なら物理ダメージを与える回数を+1する。17以上ならさらに+1する。
スキル使用者のDEXとINTの合計値が25以上なら、さらにスキルクールタイム回復量を10加算する。30以上ならさらに10加算する。
";

fn call(static_user_id: usize, con: &mut Container) -> Result<(), GameError> {
    let user = con.get_char(static_user_id)?;
    let user_name = user.static_data.name;
    let enemy = con.get_enemy();
    let enemy_name = enemy.static_data.name;
    let cooltime = 350.0;

    let skill_atk = 0.1;
    let dmg = calc_damage(user, enemy, DamageType::Physics, skill_atk);

    let mut num_attucks = 5;
    if user.agi() >= 14.0 {
        num_attucks += 1;
    }
    if user.agi() >= 17.0 {
        num_attucks += 1;
    }

    let mut skill_cooltime_heal = user.skill_cootime_heal();
    let dex_int_sum = user.dex() + user.int();
    if dex_int_sum >= 25.0 {
        skill_cooltime_heal += 10.0;
    }
    if dex_int_sum >= 30.0 {
        skill_cooltime_heal += 10.0;
    }

    let logs = [
        dmgs_msg_template(
            user_name,
            SKILL.name,
            enemy_name,
            dmg,
            DamageType::Physics,
            num_attucks,
        ),
        format!(
            "味方全員のスキルクールタイムが{}回復した",
            skill_cooltime_heal.round()
        ),
    ];

    con.update_char(static_user_id, |user| {
        user.set_skill_cooltime(SKILL.id, cooltime)?;
        user.add_hate(60.0);
        Ok(())
    })?;

    con.update_enemy(|enemy| {
        for _ in 0..num_attucks {
            enemy.accept_damage(dmg);
        }
    });

    con.update_chars(|char| {
        char.heal_skill_cooltime(skill_cooltime_heal);
    });

    con.consume_player_side_mp(SKILL.need_mp);

    con.log(logs.join(""));

    Ok(())
}
