use crate::{enemys::RuntimeEnemyId, skill::StaticSkillId, static_char::StaticCharId};

#[derive(Debug,Clone)]
pub enum GameCoreActorCommand {
    TurnEnd,
    UseSkill {
        user: StaticCharId,
        skill: StaticSkillId,
    },
    GameStart,
    ChangeFocusEnemy {
        enemy_id: RuntimeEnemyId
    }
}
