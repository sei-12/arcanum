use std::{
    any::{self, Any},
    fmt::Debug,
};

use dyn_clone::DynClone;

use crate::{
    passive::{
        DisplayPassiveInfo, PassiveRuntimeId, PassiveUpdateStateError, PassiveUpdateStateMessage,
        passive_events, status::PassiveStatus,
    },
    state::{GameState, LtId},
};

pub trait Passive: DynClone + Debug + Send + 'static {
    fn runtime_id(&self) -> PassiveRuntimeId;
    fn static_id(&self) -> any::TypeId {
        self.type_id()
    }
    fn display(&'_ self) -> Option<DisplayPassiveInfo<'_>>;

    fn should_trash(&self) -> bool;
    #[allow(unused_variables)]
    fn update_state(
        &mut self,
        msg: &PassiveUpdateStateMessage,
    ) -> Result<(), PassiveUpdateStateError> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn status(&self, status: &mut PassiveStatus) {}

    #[allow(unused_variables)]
    fn turn_start(
        &self,
        owner: LtId,
        state: &GameState,
        effects: &mut Vec<passive_events::PassiveEvent>,
    ) {
    }
}

dyn_clone::clone_trait_object!(Passive);
