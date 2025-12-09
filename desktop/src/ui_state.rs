use game_core6::runtime_id::{RuntimeCharId, RuntimeSkillId};
use std::collections::HashSet;

#[derive(Debug)]
pub struct UIState {
    opened_skill_detail: HashSet<(RuntimeCharId, RuntimeSkillId)>,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            opened_skill_detail: HashSet::new(),
        }
    }

    pub fn update(&mut self, msg: UiStateUpdateMessage) {
        match msg {
            UiStateUpdateMessage::OpenSkillDetail { char_id, skill_id } => {
                self.opened_skill_detail.insert((char_id, skill_id));
            }
            UiStateUpdateMessage::CloseSkillDetail { char_id, skill_id } => {
                self.opened_skill_detail.remove(&(char_id, skill_id));
            }
        }
    }

    pub fn is_skill_detail_opened(&self, char_id: RuntimeCharId, skill_id: RuntimeSkillId) -> bool {
        self.opened_skill_detail.contains(&(char_id, skill_id))
    }
}

#[derive(Debug, Clone)]
pub enum UiStateUpdateMessage {
    OpenSkillDetail {
        char_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
    },
    CloseSkillDetail {
        char_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
    },
}
