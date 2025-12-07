use crate::{
    LevelNum, SpNum, enemy::StaticEnemyDataInstance, lt_common::LtCommon,
    runtime_id::RuntimeEnemyId,
};


#[derive(Debug, Clone)]
pub struct ButtleEnemy {
    lt_common: LtCommon,
    runtime_id: RuntimeEnemyId,
    static_data: StaticEnemyDataInstance,
    sp: SpNum,
}

impl ButtleEnemy {
    pub(crate) fn new(
        runtime_enemy_id: RuntimeEnemyId,
        level: LevelNum,
        data: StaticEnemyDataInstance,
    ) -> Self {
        assert!(level > 0);

        let lt_common = LtCommon::new(data.potential().clone(), level);

        Self {
            lt_common,
            runtime_id: runtime_enemy_id,
            static_data: data,
            sp: 0,
        }
    }

    pub(crate) fn heal_sp(&mut self, num: SpNum) {
        self.sp = self.sp.saturating_add(num)
    }
    pub(crate) fn lt_mut(&mut self) -> &mut LtCommon {
        &mut self.lt_common
    }

    pub fn sp(&self) -> SpNum {
        self.sp
    }
    pub fn consume_sp(&mut self, num: SpNum) {
        self.sp = self.sp.saturating_sub(num);
    }

    pub fn lt(&self) -> &LtCommon {
        &self.lt_common
    }

    pub fn static_data(&self) -> &StaticEnemyDataInstance {
        &self.static_data
    }

    pub fn runtime_id(&self) -> RuntimeEnemyId {
        self.runtime_id
    }
}
