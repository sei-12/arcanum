use crate::{args::EnemyData, enemys::RuntimeEnemyId, event::EventsQuePusher, lt_common::LtCommon, state::{GameState, LtId}};

pub struct ButtleEnemy {}

impl ButtleEnemy {
    pub fn new(data: &EnemyData, id: RuntimeEnemyId) -> Self{
        todo!()

    }
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
