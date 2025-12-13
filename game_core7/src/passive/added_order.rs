use std::{
    collections::{BTreeMap, HashMap},
    ops::Add,
};

use crate::StaticPassiveId;

#[derive(Debug, Clone)]
pub(super) struct AddedOrder {
    count: u64,
    sorted: BTreeMap<u64, StaticPassiveId>,
    runtime_id_to_count: HashMap<StaticPassiveId, u64>,
}

impl AddedOrder {
    pub fn new() -> Self {
        Self {
            count: 0,
            sorted: BTreeMap::new(),
            runtime_id_to_count: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = StaticPassiveId> {
        self.sorted.values().copied()
    }

    pub fn add(&mut self, id: StaticPassiveId) {
        self.count += 1;
        let old_item = self.sorted.insert(self.count, id);
        if old_item.is_some() {
            panic!("呼び出し元は重複しないIDであることを保証しないといけない")
        }
        self.runtime_id_to_count.insert(id, self.count);
    }

    pub(super) fn remove_expect(&mut self, id: StaticPassiveId) {
        let count = self
            .runtime_id_to_count
            .remove(&id)
            .expect("呼び出し元は存在するIDであることを保証していないといけない");
        self.sorted.remove(&count);
    }
}

impl Default for AddedOrder {
    fn default() -> Self {
        Self::new()
    }
}
