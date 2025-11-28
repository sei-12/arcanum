use std::{
    any::TypeId,
    collections::{BTreeMap, HashMap, hash_map::Entry},
};

use crate::{
    event::{Event, EventsQuePusher},
    passive::{
        PassiveUpdateStateError, PassiveUpdateStateMessage, RuntimePassiveId,
        cached_status::CachedPassiveStatus, status::PassiveStatus, traits::Passive,
    },
    state::{GameState, LtId},
};

#[derive(Debug, Clone, thiserror::Error)]
pub enum PassiveListError {
    #[error("該当する要素が見つかりませんでした: id={0}")]
    NotFoundPassive(RuntimePassiveId),

    #[error("同じIDを持つ要素がすでに存在します: id={0}")]
    AlreadyExists(RuntimePassiveId),

    #[error("{0}")]
    UpdateError(#[from] PassiveUpdateStateError),
}

#[derive(Debug, Clone)]
pub struct PassiveList {
    runtime_id_map: HashMap<RuntimePassiveId, Box<dyn Passive>>,
    static_id_map: HashMap<TypeId, u16>,
    status_cache: CachedPassiveStatus,
    added_order: AddedOrder,
}

impl Default for PassiveList {
    fn default() -> Self {
        Self {
            runtime_id_map: HashMap::default(),
            static_id_map: HashMap::default(),
            status_cache: CachedPassiveStatus::new(),
            added_order: AddedOrder::new(),
        }
    }
}

impl PassiveList {
    pub fn have(&self, static_id: TypeId) -> bool {
        self.static_id_map.contains_key(&static_id)
    }

    /// 重複するruntime_idを持つ要素がすでに存在している場合はエラーになります
    /// エラーになった場合変更が加えられないことが保証されています
    pub fn add(&mut self, passive: Box<dyn Passive>) -> Result<(), PassiveListError> {
        let static_id = passive.static_id();
        let runtime_id = passive.runtime_id();

        match self.runtime_id_map.entry(passive.runtime_id()) {
            Entry::Vacant(entry) => {
                entry.insert(passive);
            }
            Entry::Occupied(_) => {
                return Err(PassiveListError::AlreadyExists(passive.runtime_id()));
            }
        }

        *self.static_id_map.entry(static_id).or_insert(1) += 1;
        self.added_order.add(runtime_id);

        Ok(())
    }

    pub fn update_state(
        &mut self,
        id: RuntimePassiveId,
        msg: &PassiveUpdateStateMessage,
    ) -> Result<(), PassiveListError> {
        let passive = self
            .runtime_id_map
            .get_mut(&id)
            .ok_or(PassiveListError::NotFoundPassive(id))?;

        passive.update_state(msg)?;
        self.status_cache.need_update();

        if passive.should_trash() {
            self.trash_passive(id);
        }

        Ok(())
    }

    fn trash_passive(&mut self, id: RuntimePassiveId) {
        debug_assert!(self.runtime_id_map.contains_key(&id));

        let passive = self.runtime_id_map.remove(&id).unwrap();
        if let Entry::Occupied(mut e) = self.static_id_map.entry(passive.static_id()) {
            let v = e.get_mut();
            *v -= 1;
            if *v == 0 {
                e.remove();
            }
        }

        self.added_order.remove_expect(id);
    }

    pub fn status(&self) -> std::cell::Ref<'_, PassiveStatus> {
        self.status_cache.get(self.runtime_id_map.values())
    }
}

//--------------------------------------------------//
//                                                  //
//                     TRIGGER                      //
//                                                  //
//--------------------------------------------------//

/// 誘発の無限ループを防ぐためにある
/// 無限ループを防ぐ仕組みを実装したとしても、そういうゲーム性は
/// 面白くないと思うから気をつける
///
/// 全ての誘発能力は1ターンに回数制限があるものとする
///
/// そう仮定した場合、全ての誘発可能性のあるイベントより先に回数カ
/// ウントを増やすイベントを実行すれば無限ループにはならないはず。
///
/// sortして前にくる値が実行優先度が高い
fn event_exec_priority(event: &Event) -> u8 {
    match event {
        Event::UpdatePassiveState {
            msg: _,
            target_id: _,
            passive_id: _,
        } => 0,
        _ => 1,
    }
}

impl PassiveList {
    pub(crate) fn trigger_turn_start(
        &self,
        owner: LtId,
        state: &GameState,
        pusher: &mut impl EventsQuePusher,
    ) {
        let mut buffer = Vec::<Event>::new();
        for runtime_id in self.added_order.iter() {
            let passive = self.runtime_id_map.get(&runtime_id).unwrap();
            debug_assert_eq!(passive.runtime_id(), runtime_id);
            passive.trigger_turn_start(owner, state, &mut buffer);
            buffer.sort_by_key(event_exec_priority);
            buffer.drain(0..buffer.len()).for_each(|event| {
                pusher.push(event);
            });
        }
    }
}

//--------------------------------------------------//
//                                                  //
//                   ADDED ORDER                    //
//                                                  //
//--------------------------------------------------//
#[derive(Debug, Clone)]
struct AddedOrder {
    count: u64,
    sorted: BTreeMap<u64, RuntimePassiveId>,
    runtime_id_to_count: HashMap<RuntimePassiveId, u64>,
}

impl AddedOrder {
    fn new() -> Self {
        Self {
            count: 0,
            sorted: BTreeMap::new(),
            runtime_id_to_count: HashMap::new(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = RuntimePassiveId> {
        self.sorted.values().copied()
    }

    fn add(&mut self, id: RuntimePassiveId) {
        self.count += 1;
        let old_item = self.sorted.insert(self.count, id);
        if old_item.is_some() {
            panic!("呼び出し元は重複しないIDであることを保証しないといけない")
        }
        self.runtime_id_to_count.insert(id, self.count);
    }

    fn remove_expect(&mut self, id: RuntimePassiveId) {
        let count = self
            .runtime_id_to_count
            .remove(&id)
            .expect("呼び出し元は存在するIDであることを保証していないといけない");
        self.sorted.remove(&count);
    }
}

//--------------------------------------------------//
//                                                  //
//                       TEST                       //
//                                                  //
//--------------------------------------------------//
#[cfg(test)]
mod tests {
    use crate::passive::list::AddedOrder;

    #[test]
    fn test_added_order() {
        let mut a = AddedOrder::new();

        for i in 0..5 {
            a.add(i);
        }

        a.remove_expect(3);

        let v = a.iter().collect::<Vec<_>>();
        assert_eq!(v, vec![0, 1, 2, 4])
    }
}
