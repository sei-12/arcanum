use crate::{
    OutputBuffer, TURN_START_HEAL_MP_NUM, TURN_START_HEAL_SP_NUM, WinOrLoseOrNextwave,
    effect::Effect,
    effector::{Effector, EffectorTrait},
    runtime_id::{RuntimeCharId, RuntimeEnemyId, RuntimeSkillId},
    skill::SkillCost,
    state::{CharData, DungeonData, GameState},
};

#[derive(Debug)]
pub(crate) struct SenderSide {
    state: GameState,
}
impl SenderSide {
    pub(crate) fn new(
        chars: Vec<CharData>,
        dungeon_data: DungeonData,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            state: GameState::new(chars, dungeon_data)?,
        })
    }

    pub(crate) fn state(&self) -> &GameState {
        &self.state
    }

    pub(crate) fn game_start(
        &mut self,
        output_buffer: &mut impl OutputBuffer,
    ) -> Result<(), WinOrLoseOrNextwave> {
        start_player_turn(&mut Effector::new(&mut self.state, output_buffer))
    }
    pub(crate) fn use_skill(
        &mut self,
        user_id: RuntimeCharId,
        target_id: Option<RuntimeEnemyId>,
        skill_id: RuntimeSkillId,
        output_buffer: &mut impl OutputBuffer,
    ) -> Result<(), WinOrLoseOrNextwave> {
        let char = self.state.get_char(user_id);
        let skill = char.get_skill(skill_id);
        assert!(skill.useable(self.state()));
        let skill_instance = skill.data().clone_instance();
        let char_runtime_id = char.runtime_id();
        let mut effector = Effector::new(&mut self.state, output_buffer);
        effector.begin_char_skill(skill_instance.info().id);
        let result = skill_instance
            .call(char_runtime_id, skill_id, target_id, &mut effector)
            .inspect_err(|_| effector.end())?;
        effector.end();

        accept_skill_cost(result, char_runtime_id, skill_id, &mut effector)?;

        Ok(())
    }

    pub(crate) fn trun_end(
        &mut self,
        output_buffer: &mut impl OutputBuffer,
    ) -> Result<(), WinOrLoseOrNextwave> {
        let mut effector = Effector::new(&mut self.state, output_buffer);

        // 敵のターンを開始
        effector.start_enemy_turn();

        let mut enemys = effector.state().enemys_with_living_check();
        while let Some(enemy) = enemys.next_living_enemy(effector.state()) {
            effector.trigger_turn_start(enemy.lt_id())?;
        }

        effector.begin_game_system();
        let mut enemys = effector.state().enemys_with_living_check();
        while let Some(enemy) = enemys.next_living_enemy(effector.state()) {
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
        while let Some(enemy) = enemys.next_living_enemy(effector.state()) {
            let enemy_skill = enemy
                .static_data()
                .select_skill(enemy.runtime_id(), effector.state());
            let enemy_runtime_id = enemy.runtime_id();
            effector.begin_enemy_skill(enemy_skill.static_id());
            enemy_skill
                .call(enemy_runtime_id, &mut effector)
                .inspect_err(|_| effector.end())?;
            effector.end();
        }

        start_player_turn(&mut effector)?;

        Ok(())
    }

    // MEMO: リファクタリング対象
    // ターンスタート時のパッシブ効果で敵が全滅したらもう一回次のウェーブに行く必要がある。
    // その処理の流れが綺麗に表現できていない
    // あとstateに対するgo_next_waveの呼び出しの場所もあまり良いとは思えない。
    //
    /// 勝ちもしくは負けならtrueを返す
    pub fn go_next_wave(&mut self, output_buffer: &mut impl OutputBuffer) -> bool {
        let mut effector = Effector::new(&mut self.state, output_buffer);

        loop {
            let result = start_player_turn(&mut effector);

            if result.is_err_and(|e| e.is_win_or_lose()) {
                break true;
            };

            if result.is_ok() {
                break false;
            }
        }
    }
}

fn start_player_turn(
    effector: &mut Effector<'_, impl OutputBuffer>,
) -> Result<(), WinOrLoseOrNextwave> {
    // プレイヤーのターンを開始
    effector.start_player_turn();

    let mut chars = effector.state().chars_with_living_check();
    while let Some(char) = chars.next_living_char(effector.state()) {
        effector.trigger_turn_start(char.lt_id())?;
    }

    effector.begin_game_system();

    effector
        .accept_effect(Effect::HealMp {
            num: TURN_START_HEAL_MP_NUM,
        })
        .inspect_err(|_| effector.end())?;

    let mut chars = effector.state().chars_with_living_check();
    while let Some(char) = chars.next_living_char(effector.state()) {
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

fn accept_skill_cost(
    result: SkillCost,
    char_runtime_id: RuntimeCharId,
    skill_id: RuntimeSkillId,
    effector: &mut Effector<'_, impl OutputBuffer>,
) -> Result<(), WinOrLoseOrNextwave> {
    effector.begin_skill_cost();

    if result.mp > 0 {
        effector
            .accept_effect(Effect::ConsumeMp { num: result.mp })
            .inspect_err(|_| effector.end())?;
    }

    if result.hate > 0 {
        effector
            .accept_effect(Effect::AddHate {
                target_id: char_runtime_id,
                num: result.hate,
            })
            .inspect_err(|_| effector.end())?;
    }

    effector
        .accept_effect(Effect::SetSkillCooldown {
            target_id: char_runtime_id,
            skill_id,
            num: result.cooldown,
        })
        .inspect_err(|_| effector.end())?;

    effector.end();

    Ok(())
}
