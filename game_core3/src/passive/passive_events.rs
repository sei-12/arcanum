use crate::passive::PassiveUpdateStateMessage;

pub enum PassiveEvent {
    UpdateState(PassiveUpdateStateMessage),
}
