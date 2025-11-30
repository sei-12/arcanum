use std::fmt::Debug;

use crate::{
    buttle_enemy::{abilitys::EnemyAbility, skill::EnemySkillDocument},
    enemys::RuntimeEnemyId,
    event::EventsQuePusher,
    potential::Potential,
    state::GameState,
};

pub mod goblin;

#[enum_dispatch::enum_dispatch]
pub trait StaticEnemyTrait: Debug + Clone {
    fn name(&self) -> &'static str;
    fn potential(&self) -> &'static Potential;
    fn abilitys(&self) -> &'static [EnemyAbility];
    fn action_text(&self) -> &'static [EnemyActionText];

    fn action(&self, enemy: RuntimeEnemyId, state: &GameState, events: &mut impl EventsQuePusher);
}

#[derive(Debug, Clone)]
#[enum_dispatch::enum_dispatch(StaticEnemyTrait)]
pub enum StaticEnemy {
    Goblin(goblin::Goblin),
}

impl StaticEnemy {
    pub(crate) fn new(id: StaticEnemyId) -> Self {
        match id {
            StaticEnemyId::Goblin => Self::Goblin(goblin::Goblin),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StaticEnemyId {
    Goblin,
}

//--------------------------------------------------//
//                                                  //
//                ENEMY ACTION TEXT                 //
//                                                  //
//--------------------------------------------------//

pub enum EnemyActionText {
    /// もし{condition}なら「{skill_doc.name}」を行い、行動を終了する。
    IfThenReturn {
        condition: &'static str,
        skill_doc: &'static EnemySkillDocument,
    },
    /// 「{0.name}」を行う。
    Normal(&'static EnemySkillDocument),
}

impl EnemyActionText {
    pub fn text(&self) -> String {
        match self {
            EnemyActionText::IfThenReturn {
                condition,
                skill_doc,
            } => {
                format!(
                    "もし{condition}なら「{}」を行い、行動を終了する",
                    skill_doc.name
                )
            }
            EnemyActionText::Normal(skill_doc) => {
                format!("「{}」を行う", skill_doc.name)
            }
        }
    }
}
