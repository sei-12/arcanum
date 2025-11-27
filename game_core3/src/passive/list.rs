use std::{
    any::TypeId,
    collections::{HashMap, hash_map::Entry},
};

use crate::passive::{
    PassiveRuntimeId, PassiveUpdateStateError, PassiveUpdateStateMessage,
    cached_status::CachedPassiveStatus, status::PassiveStatus, traits::Passive,
};

#[derive(Debug, Clone, thiserror::Error)]
pub enum PassiveListError {
    #[error("該当する要素が見つかりませんでした: id={0}")]
    NotFoundPassive(PassiveRuntimeId),

    #[error("同じIDを持つ要素がすでに存在します: id={0}")]
    AlreadyExists(PassiveRuntimeId),

    #[error("{0}")]
    UpdateError(#[from] PassiveUpdateStateError),
}

#[derive(Debug, Clone)]
pub struct PassiveList {
    runtime_id_map: HashMap<PassiveRuntimeId, Box<dyn Passive>>,
    static_id_map: HashMap<TypeId, u16>,
    status_cache: CachedPassiveStatus,
}

impl Default for PassiveList {
    fn default() -> Self {
        Self {
            runtime_id_map: HashMap::default(),
            static_id_map: HashMap::default(),
            status_cache: CachedPassiveStatus::new(),
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

        match self.runtime_id_map.entry(passive.runtime_id()) {
            Entry::Vacant(entry) => {
                entry.insert(passive);
            }
            Entry::Occupied(_) => {
                return Err(PassiveListError::AlreadyExists(passive.runtime_id()));
            }
        }

        *self.static_id_map.entry(static_id).or_insert(1) += 1;

        Ok(())
    }

    pub fn update_state(
        &mut self,
        id: PassiveRuntimeId,
        state: &PassiveUpdateStateMessage,
    ) -> Result<(), PassiveListError> {
        let passive = self
            .runtime_id_map
            .get_mut(&id)
            .ok_or(PassiveListError::NotFoundPassive(id))?;

        passive.update_state(state)?;
        self.status_cache.need_update();

        if passive.should_trash() {
            self.trash_passive(id);
        }

        Ok(())
    }

    fn trash_passive(&mut self, id: PassiveRuntimeId) {
        debug_assert!(self.runtime_id_map.contains_key(&id));

        let passive = self.runtime_id_map.remove(&id).unwrap();
        if let Entry::Occupied(mut e) = self.static_id_map.entry(passive.static_id()) {
            let v = e.get_mut();
            *v -= 1;
            if *v == 0 {
                e.remove();
            }
        }
    }

    pub fn status(&self) -> std::cell::Ref<'_, PassiveStatus> {
        self.status_cache.get(self.runtime_id_map.values())
    }
}
