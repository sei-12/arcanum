use crate::{
    OutputBuffer,
    runtime_id::{RuntimeCharId, RuntimeSkillId},
    state::GameState,
};

pub trait Effector {}


//--------------------------------------------------//
//                                                  //
//                 PASSIVE EFFECTOR                 //
//                                                  //
//--------------------------------------------------//

pub(crate) trait TriggerPassiveEffector: Effector {
    fn begin(&mut self);
    fn end(&mut self);
}

//--------------------------------------------------//
//                                                  //
//                  SKILL EFFECTOR                  //
//                                                  //
//--------------------------------------------------//
pub(crate) struct CharSkillEffector<'a, T: OutputBuffer> {
    buffer: &'a mut T,
}
impl<'a, T: OutputBuffer> CharSkillEffector<'a, T> {
    pub(crate) fn new(
        user_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
        output_buffer: &'a mut T,
    ) -> Self {
        Self {
            buffer: output_buffer,
        }
    }
}

impl<'a, T: OutputBuffer> Effector for CharSkillEffector<'a, T> {}
