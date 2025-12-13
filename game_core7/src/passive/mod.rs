use std::{
    any::Any,
    cell::Ref,
    collections::{HashMap, VecDeque, hash_map},
    fmt::Debug,
    ops::Add,
};

mod added_order;
mod cached_status;
pub mod passive_box;
pub mod status;

use crate::{
    StaticPassiveId,
    any_message::AnyMessage,
    core_actor::CtxContainer,
    damage::Damage,
    effect::Effect,
    passive::{
        added_order::AddedOrder, cached_status::CachedPassiveStatus, passive_box::PassiveBox,
        status::PassiveStatus,
    },
    runtime_id::LtId,
    state::GameState,
};

pub struct PassiveInfo {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone)]
pub struct PassiveUpdateInfo {
    need_update_cached_status: bool,
}

pub trait PassiveTrait: Debug {
    fn display(&self) -> String;
    fn clone_box(&self) -> PassiveBox;
    fn static_id(&self) -> StaticPassiveId;
    fn merge(&mut self, passive: &dyn Any) -> PassiveUpdateInfo;
    fn should_trash(&self) -> bool;
    fn update(&mut self, msg: &AnyMessage) -> PassiveUpdateInfo;
    fn info(&self) -> &PassiveInfo;
    fn frame(&self, owner: LtId, state: &GameState, effects_buffer: &mut VecDeque<Effect>);
    fn status(&self, s: &mut PassiveStatus);

    fn trigger_recv_dmg(
        &self,
        owner: LtId,
        state: &GameState,
        dmg: &Damage,
        ctx: &mut CtxContainer,
    ) {
        assert_eq!(owner, dmg.target());
    }
    fn trigger_deal_dmg(
        &self,
        owner: LtId,
        state: &GameState,
        dmg: &Damage,
        ctx: &mut CtxContainer,
    ) {
        assert_eq!(dmg.causer().to_lt_id().unwrap(), owner);
    }
}

#[derive(Debug, Clone, Default)]
pub struct PassiveList {
    added_order: AddedOrder,
    map: HashMap<StaticPassiveId, PassiveBox>,
    cached_status: CachedPassiveStatus,
}

impl PassiveList {
    pub(crate) fn add(&mut self, passive: PassiveBox) {
        assert!(!passive.should_trash());

        let need_update = match self.map.entry(passive.static_id()) {
            hash_map::Entry::Occupied(mut entry) => {
                let info = entry.get_mut().merge(&passive);

                if entry.get().should_trash() {
                    self.added_order.remove_expect(passive.static_id());
                    entry.remove();
                    true
                } else {
                    info.need_update_cached_status
                }
            }
            hash_map::Entry::Vacant(entry) => {
                self.added_order.add(passive.static_id());
                entry.insert(passive);
                true
            }
        };

        if need_update {
            self.cached_status.need_update();
        }
    }

    pub fn update_state(&mut self, static_id: StaticPassiveId, msg: &AnyMessage) {
        let hash_map::Entry::Occupied(mut entry) = self.map.entry(static_id) else {
            // 更新されて捨てられている可能性がある
            return;
        };

        let info = entry.get_mut().update(msg);
        if info.need_update_cached_status {
            self.cached_status.need_update();
        }

        if entry.get().should_trash() {
            self.added_order.remove_expect(static_id);
            entry.remove();
            self.cached_status.need_update();
        };
    }

    pub(crate) fn frame(&self, state: &GameState, ctx: &mut CtxContainer) {}

    pub(crate) fn status(&self) -> Ref<'_, PassiveStatus> {
        self.cached_status.get(self.map.values())
    }

    fn added_order_items(&self) -> impl Iterator<Item = &PassiveBox> {
        self.added_order.iter().map(|id| self.map.get(&id).unwrap())
    }

    pub(crate) fn trigger_recv_dmg(
        &self,
        owner: LtId,
        state: &GameState,
        dmg: &Damage,
        ctx: &mut CtxContainer,
    ) {
        self.added_order_items().for_each(|passive_box| {
            passive_box.trigger_recv_dmg(owner, state, dmg, ctx);
        });
    }

    pub(crate) fn trigger_deal_dmg(
        &self,
        owner: LtId,
        state: &GameState,
        dmg: &Damage,
        ctx: &mut CtxContainer,
    ) {
        self.added_order_items().for_each(|passive_box| {
            passive_box.trigger_deal_dmg(owner, state, dmg, ctx);
        });
    }
}
