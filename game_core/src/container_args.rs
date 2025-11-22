use crate::{Num, enemy_ai::StaticEnemyId, skills::StaticSkillId};

#[derive(Debug, Clone)]
pub struct CharData {
    pub level: Num,
    pub static_char_id: usize,
    pub own_skill_ids: Vec<StaticSkillId>,
}

#[derive(Debug, Clone)]
pub struct EnemyData {
    pub level: Num,
    pub static_enemy_id: StaticEnemyId,
}

#[derive(Debug, Clone)]
pub struct ContainerArgs {
    pub chars: Vec<CharData>,
    pub enemy: EnemyData,
}
