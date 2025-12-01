use crate::{enemys::RuntimeEnemyId, skill::StaticSkillId, static_char::StaticCharId};

#[derive(Debug, Clone)]
pub enum GameCoreActorCommand {
    TurnEnd,
    UseSkill {
        user_id: StaticCharId,
        skill_id: StaticSkillId,
    },
    GameStart,
    ChangeFocusEnemy {
        enemy_id: RuntimeEnemyId,
    },
}
