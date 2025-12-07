use crate::{enemy::StaticEnemyDataInstance, passive::PassiveList, runtime_id::RuntimeEnemyId};


pub struct ButtleEnemy {}
impl ButtleEnemy {
    pub(crate) fn passive_list(&self) -> &PassiveList {
        todo!()
    }
    pub(crate) fn get_mut_passive_list(&mut self) -> &mut PassiveList {
        todo!()
    }

    pub(crate) fn static_data(&self) -> &StaticEnemyDataInstance {
        todo!()
    }
    pub(crate) fn runtime_id(&self) -> RuntimeEnemyId {
        todo!()
    }
}
