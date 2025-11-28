use crate::{
    args::EnemyData,
    enemys::{ButtleEnemysItem, RuntimeEnemyId},
    event::EventsQuePusher,
    lt_common::LtCommon,
    state::{GameState, LtId},
};

#[derive(Debug)]
pub struct ButtleEnemy {}

impl ButtleEnemy {
    pub fn lt(&self) -> &LtCommon {
        todo!()
    }
    pub fn lt_mut(&mut self) -> &mut LtCommon {
        todo!()
    }
    pub fn lt_id(&self) -> LtId {
        todo!()
    }
    pub fn runtime_id(&self) -> RuntimeEnemyId {
        todo!()
    }

    pub fn play_action(&self, state: &GameState, events: &mut impl EventsQuePusher) {}
}

impl ButtleEnemysItem for ButtleEnemy {
    fn is_dead(&self) -> bool {
        self.lt().is_dead()
    }

    fn new(data: &EnemyData, id: RuntimeEnemyId) -> Self {
        todo!()
    }
}
