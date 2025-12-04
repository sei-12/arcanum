use std::collections::VecDeque;

// mod trigger_sub_effects;

use crate::{
    CooldownNum, HateNum, MpNum, PrivateMessage, SpNum, StaticEnemySkillId, StaticPassiveId,
    StaticSkillId, StatusNum, WinOrLoseOrNextwave, damage,
    living_thing::LtId,
    passive::{Passive, PassiveUpdateStateMessage},
    skill::RuntimeSkillId,
    state::{GameState, RuntimeCharId, RuntimeEnemyId, UpdateStateMessage},
};

pub struct Effecter<'a> {
    state: &'a mut GameState,
    messages: Vec<PrivateMessage>,
}

//--------------------------------------------------//
//                                                  //
//                     PRIVATE                      //
//                                                  //
//--------------------------------------------------//

fn get_sub_effects(_effect: &UpdateStateMessage, _buffer: &mut VecDeque<UpdateStateMessage>) {
    // 今は副作用はない
}

impl<'a> Effecter<'a> {
    pub(crate) fn take_messages(self) -> Vec<PrivateMessage> {
        self.messages
    }

    pub(crate) fn new(state: &'a mut GameState) -> Self {
        Self {
            state,
            messages: Vec::new(),
        }
    }

    pub(crate) fn enemy_skill_begin(&mut self, skill_id: StaticEnemySkillId) {
        self.messages
            .push(PrivateMessage::EnemySkillBegin(skill_id));
    }
    pub(crate) fn enemy_skill_end(&mut self) {
        self.messages.push(PrivateMessage::EnemySkillEnd);
    }
    pub(crate) fn skill_begin(&mut self, skill_id: StaticSkillId) {
        self.messages.push(PrivateMessage::SkillBegin(skill_id));
    }
    pub(crate) fn skill_end(&mut self) {
        self.messages.push(PrivateMessage::SkillEnd);
    }
    pub(crate) fn same_time_begin(&mut self) {
        self.messages.push(PrivateMessage::SameTimeBegin);
    }
    pub(crate) fn same_time_end(&mut self) {
        self.messages.push(PrivateMessage::SameTimeEnd);
    }

    // これ以上簡潔にしにくい
    /// すべてのメソッドから呼ばれる内部処理
    fn inner(&mut self, msg: UpdateStateMessage) -> Result<(), WinOrLoseOrNextwave> {
        // 勝ちか負けなら副作用の適用は即中断
        // 次のウェーブに行くだけなら副作用を実行

        let (frame, result) = 'block: {
            let mut frame = crate::Frame {
                main_effect: msg,
                sub_effects: Vec::new(),
            };

            let result = self.state.update(&frame.main_effect);
            if result.is_some_and(|r| r.is_win_or_lose()) {
                break 'block (frame, result);
            }

            let mut sub_effects_que = VecDeque::new();
            get_sub_effects(&frame.main_effect, &mut sub_effects_que);

            while let Some(current_msg) = sub_effects_que.pop_front() {
                let result = self.state.update(&current_msg);
                frame.sub_effects.push(current_msg);

                if result.is_some_and(|r| r.is_win_or_lose()) {
                    break 'block (frame, result);
                } else {
                    get_sub_effects(frame.sub_effects.last().unwrap(), &mut sub_effects_que);
                }
            }

            (frame, None)
        };

        self.messages.push(PrivateMessage::Frame(frame));

        match result {
            Some(r) => Err(r),
            None => Ok(()),
        }
    }
}

//--------------------------------------------------//
//                                                  //
//                      PUBLIC                      //
//                                                  //
//--------------------------------------------------//

impl<'a> Effecter<'a> {
    pub fn state(&self) -> &GameState {
        self.state
    }
}

//
// 変更する際はAIに任せるのが良さそう
//
impl<'a> Effecter<'a> {
    pub fn dmg(&mut self, dmg: damage::Damage) -> Result<(), WinOrLoseOrNextwave> {
        let msg = UpdateStateMessage::Damage(dmg);
        self.inner(msg)
    }

    pub fn heal(&mut self, target: LtId, num: StatusNum) -> Result<(), WinOrLoseOrNextwave> {
        let msg = UpdateStateMessage::HealHp(target, num);
        self.inner(msg)
    }

    pub fn consume_mp(&mut self, num: MpNum) -> Result<(), WinOrLoseOrNextwave> {
        let msg = UpdateStateMessage::ConsumeMp(num);
        self.inner(msg)
    }

    pub fn heal_mp(&mut self, num: MpNum) -> Result<(), WinOrLoseOrNextwave> {
        let msg = UpdateStateMessage::HealMp(num);
        self.inner(msg)
    }

    pub fn consume_sp(
        &mut self,
        target: RuntimeEnemyId,
        num: SpNum,
    ) -> Result<(), WinOrLoseOrNextwave> {
        let msg = UpdateStateMessage::ConsumeSp(target, num);
        self.inner(msg)
    }

    pub fn heal_sp(
        &mut self,
        target: RuntimeEnemyId,
        num: SpNum,
    ) -> Result<(), WinOrLoseOrNextwave> {
        let msg = UpdateStateMessage::HealSp(target, num);
        self.inner(msg)
    }

    pub fn update_passive_state(
        &mut self,
        target: LtId,
        passive_id: StaticPassiveId,
        message: PassiveUpdateStateMessage,
    ) -> Result<(), WinOrLoseOrNextwave> {
        let msg = UpdateStateMessage::UpdatePassiveState(target, passive_id, message);
        self.inner(msg)
    }

    pub fn add_passive(
        &mut self,
        target: LtId,
        passive: Box<dyn Passive>,
    ) -> Result<(), WinOrLoseOrNextwave> {
        let msg = UpdateStateMessage::AddPassive(target, passive);
        self.inner(msg)
    }

    pub fn set_skill_cooldown(
        &mut self,
        char_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
        cooldown: CooldownNum,
    ) -> Result<(), WinOrLoseOrNextwave> {
        let msg = UpdateStateMessage::SetSkillCooldown(char_id, skill_id, cooldown);
        self.inner(msg)
    }

    pub fn heal_skill_cooldown(
        &mut self,
        char_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
        cooldown: CooldownNum,
    ) -> Result<(), WinOrLoseOrNextwave> {
        let msg = UpdateStateMessage::HealSkillCooldown(char_id, skill_id, cooldown);
        self.inner(msg)
    }

    pub fn heal_skill_cooldown_all(
        &mut self,
        char_id: RuntimeCharId,
        cooldown: CooldownNum,
    ) -> Result<(), WinOrLoseOrNextwave> {
        let msg = UpdateStateMessage::HealSkillCooldownAll(char_id, cooldown);
        self.inner(msg)
    }

    pub fn add_hate(
        &mut self,
        char_id: RuntimeCharId,
        hate: HateNum,
    ) -> Result<(), WinOrLoseOrNextwave> {
        let msg = UpdateStateMessage::AddHate(char_id, hate);
        self.inner(msg)
    }
}
