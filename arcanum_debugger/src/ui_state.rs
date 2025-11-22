use std::{cell::RefCell, collections::HashMap};

use game_core::{chars::StaticCharId, skills::StaticSkillId};

#[derive(Debug, Clone)]
pub struct UiStateContainer {
    open_skill_window_map: RefCell<HashMap<(StaticCharId, StaticSkillId), bool>>,
}

impl UiStateContainer {
    pub fn new() -> Self {
        Self {
            open_skill_window_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_open_skill_window(&self, char_id: StaticCharId, skill_id: StaticSkillId) -> bool {
        self.open_skill_window_map
            .borrow()
            .get(&(char_id, skill_id))
            .copied()
            .unwrap_or(false)
    }

    pub fn set_open_skill_window(&self, char_id: StaticCharId, skill_id: StaticSkillId, val: bool) {
        self.open_skill_window_map
            .borrow_mut()
            .insert((char_id, skill_id), val);
    }
}
