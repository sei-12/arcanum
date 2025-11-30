use crate::{event, state::GameState};

pub(crate) enum WinOrLoseOrNextwave {
    Win,
    Lose,
    NextWave,
}

pub(crate) struct EventAccepter<'a> {
    // 将来的にこのバッファーがボトルネックになった場合、単なるVecではなくVec<Vec<Event>>にして
    // bufferを保持するbufferもeventを保持するbufferもwith_capacityで確保するとpushの際のmemcopyが減るはず
    events_buffer: Vec<event::Event>,
    state: &'a mut GameState,
}

impl<'a> EventAccepter<'a> {
    pub(crate) fn accpect(&mut self, event: event::Event) -> Result<(), WinOrLoseOrNextwave> {
        self.events_buffer.push(event.clone());
        self.state.accept_event(event);

        match self.state.check_game_end() {
            crate::state::CheckGameEndResult::GoNextWave => Err(WinOrLoseOrNextwave::NextWave),
            crate::state::CheckGameEndResult::Lose => Err(WinOrLoseOrNextwave::Lose),
            crate::state::CheckGameEndResult::Win => Err(WinOrLoseOrNextwave::Win),
            crate::state::CheckGameEndResult::None => Ok(()),
        }
    }

    pub(crate) fn get_state(&self) -> &GameState {
        self.state
    }

    // MEMO: 将来的にバッファーの実装が変わるかもしれないから、具体的な型ではなくimplにして返す
    pub(crate) fn into_iter(self) -> impl IntoIterator<Item = event::Event> {
        self.events_buffer.into_iter()
    }
}
