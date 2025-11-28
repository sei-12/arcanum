use std::collections::VecDeque;

use crate::event::{self, Event};

#[derive(Debug, Clone, Default)]
pub struct EventsQue {
    inner: VecDeque<Event>,
}
impl EventsQue {
    pub fn pop(&mut self) -> Option<Event> {
        self.inner.pop_front()
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}
impl event::EventsQuePusher for EventsQue {
    fn push(&mut self, event: crate::event::Event) {
        self.inner.push_back(event);
    }
}
