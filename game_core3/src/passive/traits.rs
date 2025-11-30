use std::{
    any::{self, Any},
    fmt::Debug,
};

use dyn_clone::DynClone;

use crate::{
    damage::Damage,
    event::Event,
    passive::{
        DisplayPassiveInfo, PassiveUpdateStateError, PassiveUpdateStateMessage, RuntimePassiveId,
        status::PassiveStatus,
    },
    state::{GameState, LtId},
};

pub trait Passive: DynClone + Debug + Any + Send + 'static {
    fn runtime_id(&self) -> RuntimePassiveId;
    fn static_id(&self) -> any::TypeId {
        self.type_id()
    }
    fn display(&'_ self) -> Option<DisplayPassiveInfo<'_>>;

    fn should_trash(&self) -> bool;

    /// 例えば「火傷」がすでに付与されているときに「火傷」を付与しようとした場合、mergeするかしないか
    /// このメソッドの戻り値はconstでなければならない
    fn should_merge_type(&self) -> bool;
    /// 引数のpassiveは同じ型(同じstatic_idを返す)ことが保証されている
    /// `assert_eq!(self.static_id(), passive.static_id());`
    fn merge(&mut self, passive: Box<dyn Passive>);

    fn merge_state(&self, buffer: &mut [u8]);

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
    fn trigger_turn_start(&self, owner_id: LtId, state: &GameState, effects: &mut Vec<Event>) {}

    #[allow(unused_variables)]
    fn trigger_recv_damage(
        &self,
        owner_id: LtId,
        state: &GameState,
        dmg: &Damage,
        effects: &mut Vec<Event>,
    ) {
    }
}

dyn_clone::clone_trait_object!(Passive);
