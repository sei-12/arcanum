use std::{
    any::{Any, TypeId},
    borrow::Cow,
    collections::{
        BTreeMap, HashMap,
        hash_map::{self},
    },
    fmt::Debug,
};

use dyn_clone::DynClone;

use crate::passive::status::PassiveStatus;

mod cached_status;
pub mod status;

#[derive(Debug, Clone)]
pub struct DisplayPassiveInfo<'a> {
    pub header: Cow<'a, str>,
    pub text: Cow<'a, str>,
}

#[derive(Debug, Clone)]
pub enum PassiveUpdateStateMessage {
    Unique(u64),
    UniqueBuffer([u8; 16]),
    UniqueBox(std::sync::Arc<dyn std::any::Any>),
}

//--------------------------------------------------//
//                                                  //
//                      TRAIT                       //
//                                                  //
//--------------------------------------------------//

#[allow(unused_variables)]
pub trait Passive: Send + DynClone + Debug + 'static {
    fn static_id(&self) -> TypeId {
        self.type_id()
    }

    fn display(&'_ self) -> Option<DisplayPassiveInfo<'_>>;
    fn should_trash(&self) -> bool;
    fn merge(&mut self, passive: &dyn Passive);
    fn merge_state(&self, buffer: &mut [u8]);
    fn update(&mut self, msg: &PassiveUpdateStateMessage);
    fn status(&self, field: &mut PassiveStatus) {}
}

dyn_clone::clone_trait_object!(Passive);

//--------------------------------------------------//
//                                                  //
//                       LIST                       //
//                                                  //
//--------------------------------------------------//

#[derive(Debug, Clone)]
pub struct PassiveList {
    map: HashMap<TypeId, Box<dyn Passive>>,
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

    pub fn add(&mut self, passive: Box<dyn Passive>) {
        assert!(!passive.should_trash());

        match self.map.entry(passive.static_id()) {
            hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().merge(passive.as_ref());

                if entry.get().should_trash() {
                    entry.remove();
                }
            }
            hash_map::Entry::Vacant(entry) => {
                self.added_order.add(passive.static_id());
                entry.insert(passive);
            }
        }

        self.cached_status.need_update();
    }

    pub fn update_state(&mut self, static_id: TypeId, msg: &PassiveUpdateStateMessage) {
        let hash_map::Entry::Occupied(mut entry) = self.map.entry(static_id) else {
            // 更新されて捨てられている可能性がある
            return;
        };

        entry.get_mut().update(msg);

        if entry.get().should_trash() {
            self.added_order.remove_expect(static_id);
            entry.remove();
        };

        self.cached_status.need_update();
    }

    pub fn status(&self) -> std::cell::Ref<'_, PassiveStatus> {
        self.cached_status.get(self.map.values())
    }
}

impl Default for PassiveList {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct AddedOrder {
    count: u64,
    sorted: BTreeMap<u64, TypeId>,
    runtime_id_to_count: HashMap<TypeId, u64>,
}

impl AddedOrder {
    fn new() -> Self {
        Self {
            count: 0,
            sorted: BTreeMap::new(),
            runtime_id_to_count: HashMap::new(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = TypeId> {
        self.sorted.values().copied()
    }

    fn add(&mut self, id: TypeId) {
        self.count += 1;
        let old_item = self.sorted.insert(self.count, id);
        if old_item.is_some() {
            panic!("呼び出し元は重複しないIDであることを保証しないといけない")
        }
        self.runtime_id_to_count.insert(id, self.count);
    }

    fn remove_expect(&mut self, id: TypeId) {
        let count = self
            .runtime_id_to_count
            .remove(&id)
            .expect("呼び出し元は存在するIDであることを保証していないといけない");
        self.sorted.remove(&count);
    }
}
