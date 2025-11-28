use crate::{MpNum, state::Side};

pub trait EventsQuePusher {
    fn push(&mut self, event: Event);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Damage,
    HealMp { side: Side, mp: MpNum },
    UpdatePassiveState,
}
