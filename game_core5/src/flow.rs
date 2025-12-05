// use crate::{
//     TURN_START_HEAL_MP_NUM, TURN_START_HEAL_SP_NUM, WinOrLoseOrNextwave, effector::EffectAccepter, skill::RuntimeSkillId, state::{ButtleEnemysItem, RuntimeCharId, RuntimeEnemyId}
// };

// pub(crate) fn use_skill(
//     effector: &mut impl EffectAccepter,
//     user_id: RuntimeCharId,
//     skill_id: RuntimeSkillId,
//     focused_enemy_id: Option<RuntimeEnemyId>,
// ) -> Result<(), WinOrLoseOrNextwave> {
//     let skill = effector.state().get_char(user_id).skills.get(skill_id);
//     let skill_fn = skill.get_skill_fn();
//     effector.skill_begin(skill.static_data().id);
//     skill_fn(user_id, focused_enemy_id, effector)?;
//     effector.skill_end();
//     Ok(())
// }

// pub(crate) fn turn_end(effector: &mut Effecter) -> Result<(), WinOrLoseOrNextwave> {
//     enemy_turn(effector)?;
//     start_player_turn(effector)?;
//     Ok(())
// }

// pub(crate) fn game_start(effector: &mut Effecter) -> Result<(), WinOrLoseOrNextwave> {
//     start_player_turn(effector)
// }

// fn start_player_turn(effector: &mut Effecter) -> Result<(), WinOrLoseOrNextwave> {
//     effector.same_time_begin();
//     effector.heal_mp(TURN_START_HEAL_MP_NUM)?;

//     let mut chars_iter = effector.state().chars_with_living_check();
//     while let Some(char) = chars_iter.next_livint_char(effector.state()) {
//         effector.heal_skill_cooldown_all(char.runtime_id(), char.cooldown_heal())?;
//     }

//     effector.same_time_end();
    
//     effector.same_time_begin();

//     effector.same_time_end();
    
//     Ok(())
// }

// fn enemy_turn(effector: &mut Effecter) -> Result<(), WinOrLoseOrNextwave> {
//     effector.same_time_begin();

//     let mut enemys_iter = effector.state().enemys_with_living_check();
//     while let Some(enemy) = enemys_iter.next_livint_enemy(effector.state()) {
//         effector.heal_sp(enemy.runtime_id(), TURN_START_HEAL_SP_NUM)?;
//     }

//     effector.same_time_end();

//     let mut enemys = effector.state().enemys_with_living_check();
//     while let Some(enemy) = enemys.next_livint_enemy(effector.state()) {
//         let skill = (enemy.static_data().select_skill_fn)(enemy.runtime_id(), effector.state());
//         let enemy_runtime_id = enemy.runtime_id();
//         effector.enemy_skill_begin(skill.id);
//         (skill.call)(enemy_runtime_id, effector)?;
//         effector.enemy_skill_end();
//     }

//     Ok(())
// }
