use std::fmt::Debug;

use crate::{
    enemys::RuntimeEnemyId, event::EventsQuePusher, potential::Potential, state::GameState,
};

pub mod goblin;

#[enum_dispatch::enum_dispatch]
pub trait StaticEnemyTrait: Debug {
    fn name(&self) -> &'static str;
    fn potential(&self) -> &'static Potential;
    fn abilitys(&self) -> &'static [EnemyAbility];
    fn action_text(&self) -> &'static [EnemyActionText];

    fn action(&self, enemy: RuntimeEnemyId, state: &GameState, events: &mut impl EventsQuePusher);
}

#[derive(Debug)]
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
    /// もし{condition}なら「{skill_name}」を行い、行動を終了する。
    IfThenReturn {
        condition: &'static str,
        skill_name: &'static str,
    },
    /// 「{0}」を行う。
    Normal(&'static str),
}

impl EnemyActionText {
    pub fn text(&self) -> String {
        match self {
            EnemyActionText::IfThenReturn {
                condition,
                skill_name,
            } => {
                format!("もし{condition}なら「{skill_name}」を行い、行動を終了する")
            }
            EnemyActionText::Normal(skill_name) => {
                format!("「{skill_name}」を行う")
            }
        }
    }
}

//--------------------------------------------------//
//                                                  //
//                     ABILITY                      //
//                                                  //
//--------------------------------------------------//
pub enum EnemyAbility {
    // 変質
    Transition,
    // 逆襲
    Revenge,
}
