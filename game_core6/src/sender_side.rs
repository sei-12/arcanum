use std::collections::VecDeque;

use crate::{
    OutputBuffer, TURN_START_HEAL_MP_NUM, TURN_START_HEAL_SP_NUM, WinOrLoseOrNextwave,
    effect::Effect,
    effector::{Effector, EffectorTrait},
    output::GameCoreOutput,
    runtime_id::{RuntimeCharId, RuntimeSkillId},
    state::GameState,
};
pub(crate) struct SenderSide {
    state: GameState,
}
impl SenderSide {
    pub(crate) fn game_start(&mut self, output_buffer: &mut impl OutputBuffer) {}
    pub(crate) fn use_skill(
        &mut self,
        user_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
        output_buffer: &mut impl OutputBuffer,
    ) {
        let char = self.state.get_char(user_id);
        let skill = char.get_skill(skill_id).clone();
        let char_runtime_id = char.runtime_id();
        let mut effector = Effector::new(&mut self.state, output_buffer);
        effector.begin_char_skill(skill.static_id(), user_id);
        skill.call(char_runtime_id, &mut effector);
    }

    pub(crate) fn trun_end(
        &mut self,
        output_buffer: &mut impl OutputBuffer,
    ) -> Result<(), WinOrLoseOrNextwave> {
        let mut effector = Effector::new(&mut self.state, output_buffer);

        // 敵のターンを開始
        effector.start_enemy_turn();

        effector.begin_game_system();
        let mut enemys = effector.state().enemys_with_living_check();
        while let Some(enemy) = enemys.next_livint_enemy(effector.state()) {
            effector
                .accept_effect(Effect::HealSp {
                    target_id: enemy.runtime_id(),
                    num: TURN_START_HEAL_SP_NUM,
                })
                .inspect_err(|_| {
                    effector.end();
                })?;
        }
        effector.end();

        // 敵がスキルを使用
        let mut enemys = effector.state().enemys_with_living_check();
        while let Some(enemy) = enemys.next_livint_enemy(effector.state()) {
            let enemy_skill = enemy
                .static_data()
                .select_skill(enemy.runtime_id(), effector.state());
            let enemy_runtime_id = enemy.runtime_id();
            effector.begin_enemy_skill(enemy_skill.static_id(), enemy_runtime_id);
            enemy_skill
                .call(enemy_runtime_id, &mut effector)
                .inspect_err(|_| effector.end())?;
        }

        // プレイヤーのターンを開始
        effector.start_player_turn();

        effector.begin_game_system();

        effector
            .accept_effect(Effect::HealMp {
                num: TURN_START_HEAL_MP_NUM,
            })
            .inspect_err(|_| effector.end())?;

        let mut chars = effector.state().chars_with_living_check();
        while let Some(char) = chars.next_livint_char(effector.state()) {
            let runtime_char_id = char.runtime_id();
            let cooldown_heal = char.skill_cooldown_heal();

            effector
                .accept_effect(Effect::HealSkillCooldownAll {
                    target_id: runtime_char_id,
                    num: cooldown_heal,
                })
                .inspect_err(|_| effector.end())?
        }
        effector.end();

        Ok(())
    }
}
