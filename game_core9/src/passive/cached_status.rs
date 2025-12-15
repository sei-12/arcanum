use std::cell::{Ref, RefCell};

use crate::passive::{PassiveBox, PassiveStatus};

#[derive(Debug, Clone)]
pub(super) struct CachedPassiveStatus {
    need_update: RefCell<bool>,
    cache: RefCell<PassiveStatus>,
}

impl CachedPassiveStatus {
    pub fn new() -> Self {
        Self {
            need_update: RefCell::new(false),
            cache: RefCell::new(PassiveStatus::default()),
        }
    }

    pub fn need_update(&mut self) {
        *self.need_update.borrow_mut() = true;
    }

    pub fn get<'a>(
        &self,
        passives: impl Iterator<Item = &'a PassiveBox>,
    ) -> Ref<'_, PassiveStatus> {
        if *self.need_update.borrow() {
            let mut cache = self.cache.borrow_mut();
            cache.reset();
            passives.for_each(|item| {
                item.status(&mut cache);
            });
            *self.need_update.borrow_mut() = false
        };
        self.cache.borrow()
    }
}
