use crate::{
    event::Event,
    event_accepter::{EventAccepter, WinOrLoseOrNextwave},
    skill::{SkillTrait, SkillTraitPrivate, StaticSkillId},
    state::chars::RuntimeCharId,
};

pub fn start_game(accepter: &mut EventAccepter) -> Result<(), WinOrLoseOrNextwave> {
    todo!()
}

pub fn end_player_turn(accepter: &mut EventAccepter) -> Result<(), WinOrLoseOrNextwave> {
    todo!()
}

/// # Panic
/// - 不正なruntime_char_id, skill_id
pub fn use_skill(
    accepter: &mut EventAccepter,
    runtime_char_id: RuntimeCharId,
    skill_id: StaticSkillId,
) -> Result<(), WinOrLoseOrNextwave> {
    let user = accepter.get_state().chars().get_char(runtime_char_id);
    let skill = user.skills.get(skill_id).unwrap();
    let skill_fn = skill.get_skill_fn();

    accepter.accpect(Event::UseSkill {
        user_name: user.static_data().name,
        skill_name: skill.static_data().document().name,
    })?;

    let result = skill_fn(accepter, runtime_char_id)?;

    result.accept_events(accepter, runtime_char_id, skill_id)?;

    todo!()
}
