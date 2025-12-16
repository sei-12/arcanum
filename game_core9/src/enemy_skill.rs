use std::collections::VecDeque;

use crate::{
    MAX_CHARACTERS, StaticEnemySkillId, StatusNum,
    damage::{Damage, DamageType},
    effect::Effect,
    game_state::GameState,
    passive::PassiveBox,
    runtime_id::LtId,
};

#[derive(Debug, Clone)]
pub struct EnemySkill {
    pub id: StaticEnemySkillId,
    pub name: &'static str,
    pub need_mp: f32,
    pub start_up_frames: u64,
    pub recovery_frame: u64,
    pub actions: Vec<(EnemySkillTarget, EnemySkillAction)>,
}

impl EnemySkill {
    pub(crate) fn run_actions(&self, state: &GameState, effects_buffer: &mut VecDeque<Effect>) {
        let mut target_ids = Vec::<LtId>::with_capacity(MAX_CHARACTERS + 1);

        for (target, action) in self.actions.iter() {
            target.push_ids(state, &mut target_ids);
            for target_id in target_ids.drain(..) {
                match action {
                    EnemySkillAction::Damage { ty, dmg_mag, count } => {
                        let dmg = match ty {
                            DamageType::Magic => {
                                Damage::new_magic_damage(state, LtId::Enemy, target_id, *dmg_mag)
                            }
                            DamageType::Physics => {
                                Damage::new_physics_damage(state, LtId::Enemy, target_id, *dmg_mag)
                            }
                            DamageType::Fixed => unimplemented!(),
                        };

                        for _ in 0..*count {
                            effects_buffer.push_back(Effect::Damage(dmg.clone()));
                        }
                    }
                    EnemySkillAction::AddPassive(passive_box) => {
                        effects_buffer.push_back(Effect::AddPassive {
                            target_id,
                            passive: passive_box.clone(),
                        });
                    }
                }
            }
        }
    }

    pub(crate) fn total_frames(&self) -> u64 {
        self.start_up_frames + self.recovery_frame
    }
}

#[derive(Debug, Clone)]
pub enum EnemySkillTarget {
    Self_,
    Single,
    Multi(u8),
    AllChar,
    AllLt,
}
impl EnemySkillTarget {
    fn push_ids(&self, state: &GameState, ids: &mut Vec<LtId>) {
        match self {
            EnemySkillTarget::Self_ => ids.push(LtId::Enemy),
            EnemySkillTarget::Single => ids.push(state.get_highest_hate_char().lt_id()),
            EnemySkillTarget::Multi(n) => {
                let chars = state.get_chars_sorted_by_hate();
                chars.iter().take(*n as usize).for_each(|c| {
                    ids.push(c.lt_id());
                });
            }
            EnemySkillTarget::AllChar => {
                state
                    .get_chars()
                    .iter()
                    .map(|c| c.lt_id())
                    .for_each(|id| ids.push(id));
            }
            EnemySkillTarget::AllLt => {
                ids.push(LtId::Enemy);
                state
                    .get_chars()
                    .iter()
                    .map(|c| c.lt_id())
                    .for_each(|id| ids.push(id));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum EnemySkillAction {
    Damage {
        ty: DamageType,
        dmg_mag: StatusNum,
        count: u8,
    },
    AddPassive(PassiveBox),
}
