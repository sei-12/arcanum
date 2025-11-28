use crate::{
    SpNum,
    args::EnemyData,
    buttle_enemy::static_datas::{StaticEnemy, StaticEnemyTrait},
    enemys::{ButtleEnemysItem, RuntimeEnemyId},
    event::EventsQuePusher,
    lt_common::LtCommon,
    state::{GameState, LtId},
};

mod skill;
pub mod static_datas;

#[derive(Debug)]
pub struct ButtleEnemy {
    sp: SpNum,
    lt_common: LtCommon,
    static_data: static_datas::StaticEnemy,
    runtime_id: RuntimeEnemyId,
}

impl ButtleEnemy {
    pub fn lt(&self) -> &LtCommon {
        &self.lt_common
    }
    pub fn lt_mut(&mut self) -> &mut LtCommon {
        &mut self.lt_common
    }
    pub fn lt_id(&self) -> LtId {
        LtId::Enemy(self.runtime_id())
    }
    pub fn runtime_id(&self) -> RuntimeEnemyId {
        self.runtime_id
    }

    pub fn sp(&self) -> SpNum {
        self.sp
    }

    pub fn play_action(&self, state: &GameState, events: &mut impl EventsQuePusher) {
        self.static_data.action(self.runtime_id(), state, events);
    }
}

impl ButtleEnemysItem<EnemyData> for ButtleEnemy {
    fn is_dead(&self) -> bool {
        self.lt().is_dead()
    }

    fn new(data: &EnemyData, id: RuntimeEnemyId) -> Self {
        let static_data = StaticEnemy::new(data.id);
        let lt_common = LtCommon::new(static_data.potential(), data.level, true);
        Self {
            sp: 0,
            lt_common,
            static_data,
            runtime_id: id,
        }
    }
}
