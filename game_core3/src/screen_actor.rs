use crate::event::Event;

pub trait ScreenActorSender {
    fn send(&mut self, event: Event);
}
