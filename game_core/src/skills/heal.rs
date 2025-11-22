use crate::{container::Container, error::GameError, skills::StaticActiveSkill};

pub const SKILL: StaticActiveSkill = StaticActiveSkill {
    id: 1,
    name: "ヒール",
    need_mp: 10.0,
    call,
};

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

    let heal_num = (user.potential().int + user.potential().dex) / 2.0;

    con.get_mut_char(target_char_id)
        .expect("bug")
        .accept_heal(heal_num);

    con.log(format!(
        "{}がヒールを発動。{}のHPを{}回復した",
        user_name,
        con.get_char(target_char_id).unwrap().static_data.name,
        heal_num.round() as u32
    ));

    Ok(())
}
