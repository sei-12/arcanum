use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use smallbox::{SmallBox, smallbox, space};

use crate::{
    StaticEnemyId, StaticEnemySkillId, WinOrLoseOrNextwave, effector::EffectorTrait,
    potential::Potential, runtime_id::RuntimeEnemyId, state::GameState,
};

//--------------------------------------------------//
//                   ENEMY SKILL                    //
//--------------------------------------------------//
pub struct EnemySkillInsance(SmallBox<dyn StaticEnemySkillData, space::S1>);
impl EnemySkillInsance {
    pub fn new(skill_data: impl StaticEnemySkillData + 'static) -> Self {
        Self(smallbox!(skill_data))
    }
}

pub trait StaticEnemySkillData {
    fn static_id(&self) -> StaticEnemySkillId;
    fn call(
        &self,
        user_id: RuntimeEnemyId,
        effector: &mut dyn EffectorTrait,
    ) -> Result<(), WinOrLoseOrNextwave>;
    fn clone(&self) -> EnemySkillInsance;
}

impl Deref for EnemySkillInsance {
    type Target = dyn StaticEnemySkillData;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl DerefMut for EnemySkillInsance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
impl Clone for EnemySkillInsance {
    fn clone(&self) -> Self {
        self.0.clone()
    }
}

//--------------------------------------------------//
//                STATIC ENEMY DATA                 //
//--------------------------------------------------//
#[derive(Debug)]
pub struct StaticEnemyDataInstance(SmallBox<dyn StaticEnemyData, space::S1>);
impl StaticEnemyDataInstance {
    pub fn new(enemy_data: impl StaticEnemyData + 'static) -> Self {
        Self(smallbox!(enemy_data))
    }
}

impl Deref for StaticEnemyDataInstance {
    type Target = dyn StaticEnemyData;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl DerefMut for StaticEnemyDataInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
impl Clone for StaticEnemyDataInstance {
    fn clone(&self) -> Self {
        self.0.clone()
    }
}

pub trait StaticEnemyData: Debug + Send + Sync{
    fn static_id(&self) -> StaticEnemyId;
    fn select_skill(&self, user_id: RuntimeEnemyId, state: &GameState) -> EnemySkillInsance;
    fn potential(&self) -> &Potential;
    fn clone(&self) -> StaticEnemyDataInstance;
}
