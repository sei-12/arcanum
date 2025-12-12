use crate::effect::Effect;

pub struct FrameResult {}

pub enum Event {}

pub enum CoreOutput {}

impl TryFrom<Effect> for CoreOutput {
    type Error = ();
    fn try_from(value: Effect) -> Result<Self, Self::Error> {
        todo!()
    }
}
