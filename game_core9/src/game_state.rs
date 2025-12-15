use crate::{
    buttle_char::ButtleChar,
    buttle_enemy::ButtleEnemy,
    buttle_skill::ButtleSkill,
    lt_common::LtCommon,
    runtime_id::{LtId, RuntimeCharId, RuntimeSkillId},
};

pub struct GameState {}

//--------------------------------------------------//
//                                                  //
//                     PRIVATE                      //
//                                                  //
//--------------------------------------------------//
impl GameState {
    pub(crate) fn new() -> Self {
        todo!()
    }
}

//--------------------------------------------------//
//                                                  //
//                      PUBLIC                      //
//                                                  //
//--------------------------------------------------//
impl GameState {
    pub fn get_char(&self, id: RuntimeCharId) -> &ButtleChar {
        todo!()
    }

    pub fn get_enemy(&self) -> &ButtleEnemy {
        todo!()
    }

    pub fn get_skill(&self, id: RuntimeSkillId) -> &ButtleSkill {
        todo!()
    }

    pub fn get_skill_mut(&mut self, id: RuntimeSkillId) -> &mut ButtleSkill {
        todo!()
    }

    pub fn get_lt(&self, id: LtId) -> &LtCommon {
        todo!()
    }
}
