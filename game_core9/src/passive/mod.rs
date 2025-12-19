use std::{
    collections::{HashMap, hash_map},
    fmt::Debug,
    ops::{Deref, DerefMut},
};

mod added_order;
mod cached_status;
pub mod status;

use downcast_rs::{Downcast, impl_downcast};
use dyn_clone::DynClone;

use crate::{
    StaticPassiveId,
    any_message::AnyMessageBox,
    core_actor::EffectsBuffer,
    damage::Damage,
    game_state::GameState,
    passive::{added_order::AddedOrder, status::PassiveStatus},
    runtime_id::LtId,
};

//--------------------------------------------------//
//                                                  //
//                       BOX                        //
//                                                  //
//--------------------------------------------------//
#[derive(Debug, Clone)]
pub struct PassiveBox(Box<dyn PassiveTrait>);
impl PassiveBox {
    pub fn new(passive: impl PassiveTrait + 'static) -> Self {
        Self(Box::new(passive))
    }
}

impl Deref for PassiveBox {
    type Target = dyn PassiveTrait;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl DerefMut for PassiveBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

//--------------------------------------------------//
//                                                  //
//                       INFO                       //
//                                                  //
//--------------------------------------------------//

pub struct PassiveInfomation {
    pub id: StaticPassiveId,
    pub name: &'static str,
    pub description: &'static str,
}

//--------------------------------------------------//
//                                                  //
//                      TRAIT                       //
//                                                  //
//--------------------------------------------------//
#[allow(unused_variables)]
pub trait PassiveTrait: Debug + Downcast + DynClone {
    fn info(&self) -> &PassiveInfomation;
    fn display(&self) -> String;
    fn should_trash(&self) -> bool;
    fn merge(&mut self, passive: &PassiveBox);
    fn tick(&self, owner: LtId, state: &GameState, effects_buffer: &mut EffectsBuffer);
    fn update(&mut self, msg: &AnyMessageBox);
    fn status(&self, status: &mut PassiveStatus) {}
    fn trigger_recv_damage(
        &self,
        owner: LtId,
        dmg: &Damage,
        state: &GameState,
        effects_buffer: &mut EffectsBuffer,
    ) {
    }
}
dyn_clone::clone_trait_object!(PassiveTrait);
impl_downcast!(PassiveTrait);

#[derive(Debug, Clone)]
pub struct PassiveList {
    map: HashMap<StaticPassiveId, PassiveBox>,
    added_order: AddedOrder,
    cached_status: cached_status::CachedPassiveStatus,
}

impl PassiveList {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            added_order: AddedOrder::new(),
            cached_status: cached_status::CachedPassiveStatus::new(),
        }
    }

    pub fn display(&self) -> impl Iterator<Item = String> {
        self.added_order
            .iter()
            .map(|id| self.map.get(&id).unwrap().display())
    }

    pub fn add(&mut self, passive: PassiveBox) {
        assert!(!passive.should_trash());

        match self.map.entry(passive.info().id) {
            hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().merge(&passive);

                if entry.get().should_trash() {
                    self.added_order.remove_expect(passive.info().id);
                    entry.remove();
                }
            }
            hash_map::Entry::Vacant(entry) => {
                self.added_order.add(passive.info().id);
                entry.insert(passive);
            }
        }

        self.cached_status.need_update();
    }

    pub fn status(&self) -> std::cell::Ref<'_, PassiveStatus> {
        self.cached_status.get(self.map.values())
    }

    fn added_order_iter(&self) -> impl Iterator<Item = &PassiveBox> {
        self.added_order.iter().map(|id| self.map.get(&id).unwrap())
    }

    pub(crate) fn tick(
        &self,
        owner_id: LtId,
        state: &GameState,
        effects_buffer: &mut EffectsBuffer,
    ) {
        self.added_order_iter().for_each(|p| {
            p.tick(owner_id, state, effects_buffer);
        });
    }

    pub(crate) fn update(&mut self, id: StaticPassiveId, msg: &AnyMessageBox) {
        let hash_map::Entry::Occupied(mut entry) = self.map.entry(id) else {
            // 見つからない場合もある
            return;
        };

        entry.get_mut().update(msg);
        if entry.get().should_trash() {
            entry.remove();
            self.added_order.remove_expect(id);
        }
        self.cached_status.need_update();
    }

    pub(crate) fn trigger_recv_damage(
        &self,
        owner: LtId,
        dmg: &Damage,
        state: &GameState,
        effects_buffer: &mut EffectsBuffer,
    ) {
        self.added_order_iter().for_each(|p| {
            p.trigger_recv_damage(owner, dmg, state, effects_buffer);
        });
    }
}

impl Default for PassiveList {
    fn default() -> Self {
        Self::new()
    }
}
