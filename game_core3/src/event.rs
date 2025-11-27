
pub trait EventsQue {
    fn push(&mut self, event: Event);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Damage
}