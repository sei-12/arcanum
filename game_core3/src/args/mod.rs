use crate::{LevelNum, skill::StaticSkillId, static_char::StaticCharId};

#[derive(Debug, Clone)]
pub struct CharData {
    pub level: LevelNum,
    pub static_char_id: StaticCharId,
    pub own_skill_ids: Vec<StaticSkillId>,
}

#[derive(Debug, Clone)]
pub struct EnemyData {
    pub level: LevelNum,
    // pub static_enemy_id: StaticEnemyId,
}

#[derive(Debug, Clone)]
pub struct ContainerArgs {
    pub chars: Vec<CharData>,
    pub enemy: Vec<Vec<EnemyData>>,
}
