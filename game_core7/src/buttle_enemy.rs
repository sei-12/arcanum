use std::{collections::VecDeque, fmt::Debug};

use crate::{
    LevelNum, buttle_enemy::enemy_box::EnemyBox, core_actor::CtxContainer, effect::Effect,
    lt_common::LtCommon, potential::Potential, runtime_id::RuntimeEnemyId, state::GameState,
};

pub struct EnemyInfomation {
    pub name: &'static str,
    pub desctiption: &'static str,
}

pub trait EnemyTrait: Debug {
    fn info(&self) -> &EnemyInfomation;
    fn frame(&self, owner: RuntimeEnemyId, state: &GameState, ctx: &mut CtxContainer);
    fn potential(&self) -> &Potential;
    fn clone_box(&self) -> EnemyBox;
}

#[derive(Debug)]
pub struct ButtleEnemy {
    id: RuntimeEnemyId,
    lt_common: LtCommon,
    enemy_box: EnemyBox,
}
impl ButtleEnemy {
    pub(crate) fn new(id: RuntimeEnemyId, level: LevelNum, enemy_box: EnemyBox) -> Self {
        ButtleEnemy {
            id,
            lt_common: LtCommon::new(enemy_box.potential().clone(), level),
            enemy_box,
        }
    }

    pub fn frame(&self, state: &GameState, ctx: &mut CtxContainer) {
        self.enemy_box.frame(self.runtime_id(), state, ctx);

        self.lt_common.passive.frame(state, ctx);
    }

    pub fn runtime_id(&self) -> RuntimeEnemyId {
        self.id
    }

    pub fn lt(&self) -> &LtCommon {
        &self.lt_common
    }

    pub(crate) fn lt_mut(&mut self) -> &mut LtCommon {
        &mut self.lt_common
    }
}

pub mod enemy_box {
    use std::ops::{Deref, DerefMut};

    use crate::buttle_enemy::EnemyTrait;

    #[derive(Debug)]
    pub struct EnemyBox(Box<dyn EnemyTrait>);

    impl EnemyBox {
        pub fn new<E: EnemyTrait + 'static>(enemy: E) -> Self {
            Self(Box::new(enemy))
        }
    }

    impl Deref for EnemyBox {
        type Target = dyn EnemyTrait;

        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }
    impl DerefMut for EnemyBox {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.0
        }
    }

    impl Clone for EnemyBox {
        fn clone(&self) -> Self {
            self.0.clone_box()
        }
    }
}
