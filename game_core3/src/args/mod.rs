use crate::{LevelNum, buttle_enemy::static_datas::StaticEnemyId, skill::StaticSkillId, static_char::StaticCharId};

#[derive(Debug, Clone)]
pub struct CharData {
    pub level: LevelNum,
    pub static_char_id: StaticCharId,
    pub own_skill_ids: Vec<StaticSkillId>,
}

#[derive(Debug, Clone)]
pub struct EnemyData {
    pub level: LevelNum,
    pub id: StaticEnemyId,
}

#[derive(Debug, Clone)]
pub struct ContainerArgs {
    pub chars: Vec<CharData>,
    pub enemy: Vec<Vec<EnemyData>>,
}
