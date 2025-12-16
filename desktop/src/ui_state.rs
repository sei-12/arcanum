use std::collections::HashSet;

use game_core9::runtime_id::RuntimeSkillId;

#[derive(Debug)]
pub struct UIState {
    opened_skill_detail: HashSet<RuntimeSkillId>,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            opened_skill_detail: HashSet::new(),
        }
    }

    pub fn update(&mut self, msg: UiStateUpdateMessage) {
        match msg {
            UiStateUpdateMessage::OpenSkillDetail { skill_id } => {
                self.opened_skill_detail.insert(skill_id);
            }
            UiStateUpdateMessage::CloseSkillDetail { skill_id } => {
                self.opened_skill_detail.remove(&skill_id);
            }
        }
    }

    pub fn is_skill_detail_opened(&self, skill_id: RuntimeSkillId) -> bool {
        self.opened_skill_detail.contains(&skill_id)
    }
}

#[derive(Debug, Clone)]
pub enum UiStateUpdateMessage {
    OpenSkillDetail { skill_id: RuntimeSkillId },
    CloseSkillDetail { skill_id: RuntimeSkillId },
}
