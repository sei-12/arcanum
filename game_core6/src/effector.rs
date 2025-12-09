use std::collections::VecDeque;

use crate::{
    OutputBuffer, StaticEnemySkillId, StaticPassiveId, StaticSkillId, WinOrLoseOrNextwave,
    effect::Effect,
    output::{self, EffectedBy, GameCoreOutput},
    state::GameState,
};

pub trait EffectorTrait {
    fn state(&self) -> &GameState;
    fn accept_effect(&mut self, effect: Effect) -> Result<(), WinOrLoseOrNextwave>;
}

//--------------------------------------------------//
//                                                  //
//                 PASSIVE EFFECTOR                 //
//                                                  //
//--------------------------------------------------//

pub struct PassiveEffector<'a> {
    buffer: &'a mut VecDeque<(EffectedBy, Effect)>,
    current_exec_priority: u8,
    // 開始済みか否かはこの変数に値が入っているかどうかで判断する
    effected_by: Option<EffectedBy>,
}

impl<'a> PassiveEffector<'a> {
    pub(crate) fn new(buffer: &'a mut VecDeque<(EffectedBy, Effect)>) -> Self {
        Self {
            buffer,
            current_exec_priority: 0,
            effected_by: None,
        }
    }

    fn begined(&self) -> bool {
        self.effected_by.is_some()
    }

    pub(crate) fn begin(&mut self, id: StaticPassiveId) {
        assert!(!self.begined());
        self.effected_by = Some(EffectedBy::SubEffect(id));
        self.current_exec_priority = 0;
    }

    pub(crate) fn end(&mut self) {
        assert!(self.begined());
        self.effected_by = None;
    }

    pub fn accept_effect(&mut self, effect: Effect) {
        assert!(self.begined());

        let exec_priority = Self::exec_priority(&effect);
        assert!(self.current_exec_priority <= exec_priority);

        self.current_exec_priority = exec_priority;
        self.buffer.push_back((self.effected_by.unwrap(), effect));
    }

    fn exec_priority(effect: &Effect) -> u8 {
        match effect {
            Effect::UpdatePassiveState {
                target_id: _,
                passive_id: _,
                message: _,
            } => 1,
            _ => 2,
        }
    }
}

//--------------------------------------------------//
//                                                  //
//                     EFFECTOR                     //
//                                                  //
//--------------------------------------------------//
pub(crate) struct Effector<'a, T: OutputBuffer> {
    buffer: &'a mut T,
    state: &'a mut GameState,
    current_effected_by: Option<EffectedBy>,
}

impl<'a, T: OutputBuffer> Effector<'a, T> {
    pub(crate) fn new(state: &'a mut GameState, buffer: &'a mut T) -> Self {
        Self {
            buffer,
            state,
            current_effected_by: None,
        }
    }

    fn begined(&self) -> bool {
        self.current_effected_by.is_some()
    }

    // もうほぼ書き終わったから今から修正するほどではないけど、もしこれを呼び出す側を書き直す機会があるなら
    // 現在のbegin/endを呼び出す方式をやめて、caller的な構造体を渡すようにした方が良いと思う。callerの
    // 作成と同時にbeginをしてdropでendって感じ。

    pub(crate) fn begin_skill_cost(&mut self) {
        assert!(self.current_effected_by.is_none());
        self.current_effected_by = Some(EffectedBy::SkillCost)
    }

    pub(crate) fn begin_char_skill(&mut self, id: StaticSkillId) {
        assert!(self.current_effected_by.is_none());
        self.current_effected_by = Some(EffectedBy::CharSkill(id));
        self.buffer
            .push(GameCoreOutput::Event(output::Event::CharUseSkill));
    }

    pub(crate) fn end(&mut self) {
        assert!(self.current_effected_by.is_some());
        self.current_effected_by = None;
    }

    pub(crate) fn begin_enemy_skill(&mut self, id: StaticEnemySkillId) {
        assert!(self.current_effected_by.is_none());
        self.current_effected_by = Some(EffectedBy::EnemySkill(id));
        self.buffer
            .push(GameCoreOutput::Event(output::Event::EnemyUseSkill));
    }

    pub(crate) fn begin_game_system(&mut self) {
        assert!(self.current_effected_by.is_none());
        self.current_effected_by = Some(EffectedBy::GameSystem);
    }

    pub(crate) fn start_enemy_turn(&mut self) {
        self.buffer
            .push(GameCoreOutput::Event(output::Event::EnemyTurnStart));
    }
    pub(crate) fn start_player_turn(&mut self) {
        self.buffer
            .push(GameCoreOutput::Event(output::Event::PlayerTurnStart));
    }
}

impl<'a, T: OutputBuffer> EffectorTrait for Effector<'a, T> {
    fn state(&self) -> &GameState {
        self.state
    }

    fn accept_effect(&mut self, effect: Effect) -> Result<(), WinOrLoseOrNextwave> {
        assert!(self.begined());

        let mut effects_buffer = VecDeque::from([(self.current_effected_by.unwrap(), effect)]);

        while let Some((effected_by, effect)) = effects_buffer.pop_front() {
            let result = self.state.accept(&effect);
            if !result.accepted {
                continue;
            }
            get_trigger_effect(&effect, &mut effects_buffer, self.state);
            self.buffer
                .push(GameCoreOutput::Effect(effected_by, effect));
        }

        let result = self.state.check_win_or_lose();

        if let Err(res) = result {
            match res {
                WinOrLoseOrNextwave::GoNextwave => {
                    self.state.go_next_wave();
                    self.buffer
                        .push(GameCoreOutput::Event(output::Event::GoNextWave));
                }
                WinOrLoseOrNextwave::Lose => {
                    self.buffer.push(GameCoreOutput::Event(output::Event::Lose));
                }
                WinOrLoseOrNextwave::Win => {
                    self.buffer.push(GameCoreOutput::Event(output::Event::Win));
                }
            }
        }

        result
    }
}

fn get_trigger_effect(
    trigger: &Effect,
    buffer: &mut VecDeque<(EffectedBy, Effect)>,
    state: &GameState,
) {
    let mut effector = PassiveEffector::new(buffer);

    #[allow(clippy::single_match)]
    match trigger {
        Effect::Damage(dmg) => {
            let target = state.get_lt(dmg.target());
            target
                .passive
                .trigger_recv_damage(dmg.target(), dmg, state, &mut effector);
        }
        _ => {}
    }
}
