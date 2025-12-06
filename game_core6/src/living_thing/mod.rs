use crate::{
    passive::PassiveList,
    runtime_id::{RuntimeCharId, RuntimeSkillId},
    skill::SkillInstance,
};

pub struct ButtleChar {}
impl ButtleChar {
    pub(crate) fn passive_list(&self) -> &PassiveList {
        todo!()
    }
    pub(crate) fn get_mut_passive_list(&mut self) -> &mut PassiveList {
        todo!()
    }

    pub fn runtime_id(&self) -> RuntimeCharId {
        todo!()
    }

    pub(crate) fn get_skill(&self, id: RuntimeSkillId) -> &SkillInstance {
        assert_eq!(id.char_id, self.runtime_id());
        todo!()
    }
}

pub struct ButtleEnemy {}
impl ButtleEnemy {
    pub(crate) fn passive_list(&self) -> &PassiveList {
        todo!()
    }
    pub(crate) fn get_mut_passive_list(&mut self) -> &mut PassiveList {
        todo!()
    }
}
