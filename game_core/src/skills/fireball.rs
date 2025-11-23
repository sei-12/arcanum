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
    need_mp: 70.0,
    call,
    text: TEXT,
};

const TEXT: &str = "消費MP 70
クールタイム 3ターン
ヘイト値 70
敵にスキルダメージ1.1の魔法ダメージを与える。
スキル使用者のINTが3以下ならさらに追加でMPを10消費する。
スキル使用者のDEXが4以下ならスキル使用者に2ターンの火傷を付与する。
スキル使用者のAGIが12以上ならこのスキルのクールタイムは1小さくなる。
";

fn call(static_user_id: usize, con: &mut Container) -> Result<(), GameError> {
    let user = con.get_char(static_user_id)?;
    let user_name = user.static_data.name;
    let enemy = con.get_enemy();
    let enemy_name = enemy.static_data.name;
    let skill_atk = 1.1;
    let dmg = calc_damage(user, enemy, DamageType::Magic, skill_atk);

    let burn: Option<public_passive::Burn> = if user.dex() <= 4.0 {
        Some(public_passive::Burn::new(2))
    } else {
        None
    };

    let mut cooltime = 3;
    if user.agi() >= 12.0 {
        cooltime -= 1;
    }

    let mut addtional_need_mp = 0.0;
    if user.int() <= 3.0 {
        addtional_need_mp += 10.0;
    }

    con.update_char(static_user_id, |user| {
        if let Some(burn) = burn {
            user.passive.add(Box::new(burn));
        };
        user.set_skill_cooltime(SKILL.id, cooltime)?;
        user.add_hate(70.0);
        Ok(())
    })?;

    con.update_enemy(|enemy| {
        enemy.accept_damage(dmg);
    });

    con.consume_player_side_mp(SKILL.need_mp + addtional_need_mp);

    con.log(dmg_msg_template(
        user_name,
        SKILL.name,
        enemy_name,
        dmg,
        DamageType::Magic,
    ));

    Ok(())
}
