use crate::{
    event::{self, Event, EventsQuePusher},
    state::GameState,
};

pub(crate) enum WinOrLoseOrNextwave {
    Win,
    Lose,
    NextWave,
}

pub(crate) struct EventAccepter {
    // # performance
    // 将来的にこのバッファーがボトルネックになった場合、単なるVecではなくVec<Vec<Event>>にして
    // bufferを保持するbufferもeventを保持するbufferもwith_capacityで確保するとpushの際のmemcopyが減るはず
    events_buffer: Vec<event::Event>,
    cursor: usize,
}

impl EventsQuePusher for EventAccepter {
    fn push_event(&mut self, event: Event) {
        self.events_buffer.push(event);
    }
}

impl EventAccepter {
    pub(crate) fn new() -> Self {
        Self {
            events_buffer: Vec::new(),
            cursor: 0,
        }
    }

    // MEMO: EventsQuePusherのpush_eventメソッドがあるから、このメソッドはけしていいかも
    pub(crate) fn push_to_tmp(&mut self, event: event::Event) {
        self.events_buffer.push(event);
    }

    pub(crate) fn accpect(
        &mut self,
        event: event::Event,
        state: &mut GameState,
    ) -> Result<(), WinOrLoseOrNextwave> {
        self.events_buffer.push(event);
        self.flush(state)
    }

    pub(crate) fn flush(&mut self, state: &mut GameState) -> Result<(), WinOrLoseOrNextwave> {
        loop {
            if self.cursor == self.events_buffer.len() {
                break;
            }

            let event = self.events_buffer[self.cursor].clone();
            state.accept_event(event.clone());
            self.cursor += 1;

            if let Some(result) = match state.check_game_end() {
                crate::state::CheckGameEndResult::GoNextWave => Some(WinOrLoseOrNextwave::NextWave),
                crate::state::CheckGameEndResult::Lose => Some(WinOrLoseOrNextwave::Lose),
                crate::state::CheckGameEndResult::Win => Some(WinOrLoseOrNextwave::Win),
                crate::state::CheckGameEndResult::None => None,
            } {
                return Err(result);
            };

            #[allow(clippy::single_match)]
            match &event {
                Event::Damage(dmg) => {
                    let receiver = state.get_lt(dmg.target());
                    receiver.passive.trigger_recv_damage(
                        dmg.target(),
                        state,
                        dmg,
                        &mut self.events_buffer,
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }

    // MEMO: 将来的にバッファーの実装が変わるかもしれないから、具体的な型ではなくimplにして返す
    pub(crate) fn events(self) -> impl IntoIterator<Item = event::Event> {
        self.events_buffer.into_iter()
    }
}
