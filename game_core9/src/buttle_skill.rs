use crate::{TimeNum, game_state::GameState, runtime_id::RuntimeSkillId, skill::SkillBox};

#[derive(Debug)]
pub struct ButtleSkill {
    runtime_id: RuntimeSkillId,
    skill_box: SkillBox,
    cooldown: TimeNum,
}
impl ButtleSkill {
    pub(crate) fn new(runtime_id: RuntimeSkillId, skill_box: SkillBox) -> Self {
        Self {
            runtime_id,
            skill_box,
            cooldown: 0.0,
        }
    }

    pub fn useable(&self, state: &GameState) -> bool {
        let custom_useable = self.skill_box.custom_useable(self.runtime_id, state);
        let mp_ok = state.get_char(self.runtime_id.char_id).lt().mp()
            >= self.skill_box.need_mp(self.runtime_id, state);
        let cooldown_ok = self.cooldown <= 0.0;

        match custom_useable {
            crate::skill::SkillCustomUseable::Strong(useable) => useable,
            crate::skill::SkillCustomUseable::IgnoreNeedMp => cooldown_ok,
            crate::skill::SkillCustomUseable::IgnoreCooldown => mp_ok,
            crate::skill::SkillCustomUseable::Normal => cooldown_ok && mp_ok,
        }
    }

    pub fn heal_cooldown(&mut self, num: TimeNum) {
        self.cooldown -= num;
    }

    pub fn add_cooldown(&mut self, num: TimeNum) {
        self.cooldown += num;
    }

    pub fn skill_box(&self) -> &SkillBox {
        &self.skill_box
    }

    pub(crate) fn skill_box_mut(&mut self) -> &mut SkillBox {
        &mut self.skill_box
    }

    pub fn runtime_id(&self) -> RuntimeSkillId {
        self.runtime_id
    }
}
