use crate::{
    TURN_START_HEAL_MP_NUM, TURN_START_HEAL_SP_NUM,
    event::Event,
    event_accepter::{EventAccepter, WinOrLoseOrNextwave},
    skill::{SkillTrait, SkillTraitPrivate, StaticSkillId},
    state::{GameState, chars::RuntimeCharId},
};

pub fn start_game(
    accepter: &mut EventAccepter,
    state: &mut GameState,
) -> Result<(), WinOrLoseOrNextwave> {
    player_turn_start(accepter, state)
}

pub fn end_player_turn(
    accepter: &mut EventAccepter,
    state: &mut GameState,
) -> Result<(), WinOrLoseOrNextwave> {
    enemy_turn_start(accepter, state)?;
    enemy_turn(accepter, state)?;
    player_turn_start(accepter, state)?;
    Ok(())
}

/// # Panic
/// - 不正なruntime_char_id, skill_id
pub fn use_skill(
    accepter: &mut EventAccepter,
    state: &mut GameState,
    runtime_char_id: RuntimeCharId,
    skill_id: StaticSkillId,
) -> Result<(), WinOrLoseOrNextwave> {
    let user = state.chars().get_char(runtime_char_id);
    let skill = user.skills.get(skill_id).unwrap();
    let skill_fn = skill.get_skill_fn();

    accepter.accpect(
        Event::UseSkill {
            user_name: user.static_data().name,
            skill_name: skill.static_data().document().name,
        },
        state,
    )?;

    let result = skill_fn(accepter, state, runtime_char_id)?;

    result.accept_events(accepter, state, runtime_char_id, skill_id)?;

    Ok(())
}

fn player_turn_start(
    accepter: &mut EventAccepter,
    state: &mut GameState,
) -> Result<(), WinOrLoseOrNextwave> {
    accepter.accpect(Event::TurnStart(crate::state::Side::Player), state)?;
    accepter.accpect(
        Event::HealMp {
            mp: TURN_START_HEAL_MP_NUM,
        },
        state,
    )?;
    state.chars().chars().iter().for_each(|char| {
        accepter.push_to_tmp(Event::HeallSkillCooldownAll {
            char_id: char.runtime_id(),
            heal_num: char.cooldown_heal(),
        });
    });
    accepter.flush(state)?;
    state.chars().chars().iter().for_each(|char| {
        char.lt()
            .passive
            .trigger_turn_start(char.lt_id(), state, accepter);
    });

    accepter.flush(state)
}

fn enemy_turn_start(
    accepter: &mut EventAccepter,
    state: &mut GameState,
) -> Result<(), WinOrLoseOrNextwave> {
    accepter.accpect(Event::TurnStart(crate::state::Side::Enemy), state)?;
    state
        .enemys()
        .current_wave_living_enemys()
        .for_each(|enemy| {
            accepter.push_to_tmp(Event::HealSp {
                enemy_id: enemy.runtime_id(),
                num: TURN_START_HEAL_SP_NUM,
            });
        });
    accepter.flush(state)?;

    state
        .enemys()
        .current_wave_living_enemys()
        .for_each(|enemy| {
            enemy
                .lt()
                .passive
                .trigger_turn_start(enemy.lt_id(), state, accepter);
        });

    accepter.flush(state)
}

fn enemy_turn(
    accepter: &mut EventAccepter,
    state: &mut GameState,
) -> Result<(), WinOrLoseOrNextwave> {
    enemy_turn_start(accepter, state)?;

    let mut enemys = state.enemys().current_wave_enemys_with_check_living();
    while let Some(enemy) = enemys.next_living_enemy(state.enemys()) {
        (enemy.static_data().action_fn)(enemy.runtime_id(), accepter, state)?;
    }
    Ok(())
}
