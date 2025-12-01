use std::fmt::Debug;

use crate::{
    buttle_enemy::{abilitys::EnemyAbility, skill::EnemySkillDocument},
    enemys::RuntimeEnemyId,
    event_accepter::{EventAccepter, WinOrLoseOrNextwave},
    potential::Potential,
    state::GameState,
};

pub mod goblin;

pub(crate) fn get_static_enemy_data(id: StaticEnemyId) -> &'static StaticEnemyData {
    match id {
        StaticEnemyId::Goblin => &goblin::DATA,
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct StaticEnemyData {
    pub name: &'static str,
    pub potential: &'static Potential,
    pub abilitys: &'static [EnemyAbility],
    pub action_text: &'static [EnemyActionText],
    pub(crate) action_fn:
        fn(RuntimeEnemyId, &mut EventAccepter, &mut GameState) -> Result<(), WinOrLoseOrNextwave>,
}
