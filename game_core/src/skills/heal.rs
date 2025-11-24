use crate::{container::Container, error::GameError, skills::StaticActiveSkill};

pub const SKILL: StaticActiveSkill = StaticActiveSkill {
    id: 1,
    name: "ヒール",
    need_mp: 60.0,
    call,
    text: TEXT,
};

const TEXT: &str = "消費MP 60
クールタイム 300
ヘイト値 60
N = (INT + DEX) / 2 * (LEVEL + 10)
残りHP割合が最も小さい味方一人のHPをN回復する。
スキル使用者のAGIが12以上ならこのスキルのクールタイムは100小さくなる。
";

fn call(static_user_id: usize, con: &mut Container) -> Result<(), GameError> {
    let user = con.get_char(static_user_id)?;
    let user_name = user.static_data.name;
    // 最も残りHP割合の小さいキャラクター
    let target_char_id = con
        .get_chars()
        .iter()
        .min_by(|a, b| {
            let a_ = a.hp / a.max_hp();
            let b_ = b.hp / b.max_hp();
            a_.partial_cmp(&b_).expect("failed to cmp")
        })
        .expect("bug")
        .static_data
        .id;

    let heal_num = (user.int() + user.dex()) / 2.0 * (user.level + 10.0);

    let mut cooltime = 300.0;
    if user.agi() >= 12.0 {
        cooltime -= 100.0;
    }

    con.update_char(static_user_id, |user| {
        user.set_skill_cooltime(SKILL.id, cooltime)?;
        user.add_hate(60.0);
        Ok(())
    })?;

    con.update_char(target_char_id, |target| {
        target.accept_heal(heal_num);
        Ok(())
    })?;

    con.consume_player_side_mp(SKILL.need_mp);

    con.log(format!(
        "{}がヒールを発動。{}のHPを{}回復した",
        user_name,
        con.get_char(target_char_id).unwrap().static_data.name,
        heal_num.round() as u32
    ));

    Ok(())
}
