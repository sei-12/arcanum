use std::collections::VecDeque;

use crate::{
    CooldownNum, HateNum, MpNum, SpNum, StaticEnemySkillId, StaticPassiveId, StaticSkillId,
    StatusNum, WinOrLoseOrNextwave, damage,
    effect::Effect,
    game_core_output_receiver::{CoreMessage, EffectedBy},
    living_thing::LtId,
    passive::{Passive, PassiveUpdateStateMessage},
    skill::RuntimeSkillId,
    state::{GameState, RuntimeCharId, RuntimeEnemyId},
};

pub trait EffectAccepter {
    fn accept(&mut self, effect: Effect) -> Result<(), WinOrLoseOrNextwave>;
}

enum CurrentEffectCauser {}

pub(crate) struct Accepter<'a> {
    state: &'a mut GameState,
    message_buffer: Vec<CoreMessage>,
    current_effect_causer: Option<EffectedBy>,
}

impl<'a> Accepter<'a> {
    fn skill_begin(&mut self, skill_id: RuntimeSkillId) {}
}

impl<'a> EffectAccepter for Accepter<'a> {
    fn accept(&mut self, effect: Effect) -> Result<(), WinOrLoseOrNextwave> {
        let current_effect_causer = self.current_effect_causer.unwrap();
        assert!(!matches!(current_effect_causer, EffectedBy::SubEffect));

        let result = self.state.accept_effect(&effect);
        if !result.accepted {
            return result.result;
        }

        let mut sub_effects_buffer = VecDeque::<Effect>::new();
        get_sub_effects(&effect, &mut sub_effects_buffer);

        if result.accepted {
            self.message_buffer
                .push(CoreMessage::Effect(current_effect_causer, effect));
        }

        while let Some(current_effect) = sub_effects_buffer.pop_front() {
            self.state.accept_effect(&current_effect);
            get_sub_effects(&current_effect, &mut sub_effects_buffer);
        }

        self.state.win_or_lose_or_go_next_wave()
    }
}

fn get_sub_effects(_effect: &Effect, _buffer: &mut VecDeque<Effect>) {
    // 今は副作用はない
}

// use std::{any::type_name, collections::VecDeque};

// // mod trigger_sub_effects;

// use crate::{
//     CooldownNum, HateNum, MpNum, PrivateMessage, SpNum, StaticEnemySkillId, StaticPassiveId,
//     StaticSkillId, StatusNum, WinOrLoseOrNextwave, damage,
//     living_thing::LtId,
//     passive::{Passive, PassiveUpdateStateMessage},
//     skill::RuntimeSkillId,
//     state::{GameState, RuntimeCharId, RuntimeEnemyId, UpdateStateMessage},
// };

// pub struct Effecter<'a> {
//     state: &'a mut GameState,
//     messages: Vec<PrivateMessage>,
// }

// //--------------------------------------------------//
// //                                                  //
// //                     PRIVATE                      //
// //                                                  //
// //--------------------------------------------------//

// impl<'a> Effecter<'a> {
//     pub(crate) fn take_messages(self) -> Vec<PrivateMessage> {
//         self.messages
//     }

//     pub(crate) fn new(state: &'a mut GameState) -> Self {
//         Self {
//             state,
//             messages: Vec::new(),
//         }
//     }

//     pub(crate) fn enemy_skill_begin(&mut self, skill_id: StaticEnemySkillId) {
//         self.messages
//             .push(PrivateMessage::EnemySkillBegin(skill_id));
//     }
//     pub(crate) fn enemy_skill_end(&mut self) {
//         self.messages.push(PrivateMessage::EnemySkillEnd);
//     }
//     pub(crate) fn skill_begin(&mut self, skill_id: StaticSkillId) {
//         self.messages.push(PrivateMessage::SkillBegin(skill_id));
//     }
//     pub(crate) fn skill_end(&mut self) {
//         self.messages.push(PrivateMessage::SkillEnd);
//     }
//     pub(crate) fn same_time_begin(&mut self) {
//         self.messages.push(PrivateMessage::SameTimeBegin);
//     }
//     pub(crate) fn same_time_end(&mut self) {
//         self.messages.push(PrivateMessage::SameTimeEnd);
//     }

//     // これ以上簡潔にしにくい
//     /// すべてのメソッドから呼ばれる内部処理
//     fn inner(&mut self, msg: UpdateStateMessage) -> Result<(), WinOrLoseOrNextwave> {
//         // 勝ちか負けなら副作用の適用は即中断
//         // 次のウェーブに行くだけなら副作用を実行

//         let (frame, result) = 'block: {
//             let mut frame = crate::Frame {
//                 main_effect: msg,
//                 sub_effects: Vec::new(),
//             };

//             let result = self.state.update(&frame.main_effect);
//             if result.is_some_and(|r| r.is_win_or_lose()) {
//                 break 'block (frame, result);
//             }

//             let mut sub_effects_que = VecDeque::new();
//             get_sub_effects(&frame.main_effect, &mut sub_effects_que);

//             while let Some(current_msg) = sub_effects_que.pop_front() {
//                 let result = self.state.update(&current_msg);
//                 frame.sub_effects.push(current_msg);

//                 if result.is_some_and(|r| r.is_win_or_lose()) {
//                     break 'block (frame, result);
//                 } else {
//                     get_sub_effects(frame.sub_effects.last().unwrap(), &mut sub_effects_que);
//                 }
//             }

//             (frame, None)
//         };

//         self.messages.push(PrivateMessage::Frame(frame));

//         match result {
//             Some(r) => Err(r),
//             None => Ok(()),
//         }
//     }
// }

// //--------------------------------------------------//
// //                                                  //
// //                      PUBLIC                      //
// //                                                  //
// //--------------------------------------------------//

// impl<'a> Effecter<'a> {
//     pub fn state(&self) -> &GameState {
//         self.state
//     }
// }

// //
// // 変更する際はAIに任せるのが良さそう
// //
// impl<'a> Effecter<'a> {
//     pub fn dmg(&mut self, dmg: damage::Damage) -> Result<(), WinOrLoseOrNextwave> {
//         let msg = UpdateStateMessage::Damage(dmg);
//         self.inner(msg)
//     }

//     pub fn heal(&mut self, target: LtId, num: StatusNum) -> Result<(), WinOrLoseOrNextwave> {
//         let msg = UpdateStateMessage::HealHp(target, num);
//         self.inner(msg)
//     }

//     pub fn consume_mp(&mut self, num: MpNum) -> Result<(), WinOrLoseOrNextwave> {
//         let msg = UpdateStateMessage::ConsumeMp(num);
//         self.inner(msg)
//     }

//     pub fn heal_mp(&mut self, num: MpNum) -> Result<(), WinOrLoseOrNextwave> {
//         let msg = UpdateStateMessage::HealMp(num);
//         self.inner(msg)
//     }

//     pub fn consume_sp(
//         &mut self,
//         target: RuntimeEnemyId,
//         num: SpNum,
//     ) -> Result<(), WinOrLoseOrNextwave> {
//         let msg = UpdateStateMessage::ConsumeSp(target, num);
//         self.inner(msg)
//     }

//     pub fn heal_sp(
//         &mut self,
//         target: RuntimeEnemyId,
//         num: SpNum,
//     ) -> Result<(), WinOrLoseOrNextwave> {
//         let msg = UpdateStateMessage::HealSp(target, num);
//         self.inner(msg)
//     }

//     pub fn update_passive_state(
//         &mut self,
//         target: LtId,
//         passive_id: StaticPassiveId,
//         message: PassiveUpdateStateMessage,
//     ) -> Result<(), WinOrLoseOrNextwave> {
//         let msg = UpdateStateMessage::UpdatePassiveState(target, passive_id, message);
//         self.inner(msg)
//     }

//     pub fn add_passive(
//         &mut self,
//         target: LtId,
//         passive: Box<dyn Passive>,
//     ) -> Result<(), WinOrLoseOrNextwave> {
//         let msg = UpdateStateMessage::AddPassive(target, passive);
//         self.inner(msg)
//     }

//     pub fn set_skill_cooldown(
//         &mut self,
//         char_id: RuntimeCharId,
//         skill_id: RuntimeSkillId,
//         cooldown: CooldownNum,
//     ) -> Result<(), WinOrLoseOrNextwave> {
//         let msg = UpdateStateMessage::SetSkillCooldown(char_id, skill_id, cooldown);
//         self.inner(msg)
//     }

//     pub fn heal_skill_cooldown(
//         &mut self,
//         char_id: RuntimeCharId,
//         skill_id: RuntimeSkillId,
//         cooldown: CooldownNum,
//     ) -> Result<(), WinOrLoseOrNextwave> {
//         let msg = UpdateStateMessage::HealSkillCooldown(char_id, skill_id, cooldown);
//         self.inner(msg)
//     }

//     pub fn heal_skill_cooldown_all(
//         &mut self,
//         char_id: RuntimeCharId,
//         cooldown: CooldownNum,
//     ) -> Result<(), WinOrLoseOrNextwave> {
//         let msg = UpdateStateMessage::HealSkillCooldownAll(char_id, cooldown);
//         self.inner(msg)
//     }

//     pub fn add_hate(
//         &mut self,
//         char_id: RuntimeCharId,
//         hate: HateNum,
//     ) -> Result<(), WinOrLoseOrNextwave> {
//         let msg = UpdateStateMessage::AddHate(char_id, hate);
//         self.inner(msg)
//     }
// }

// //--------------------------------------------------//
// //                                                  //
// //              PASSIVE EFFECT BUFFER               //
// //                                                  //
// //--------------------------------------------------//

// /// そう仮定した場合、全ての誘発可能性のあるイベントより先に回数カ
// /// ウントを増やすイベントを実行すれば無限ループにはならないはず。
// // ソートするのではなく、ソートしなくても良い順序で挿入されることを強制する
// // 毎回ソートしてると、ソートするためのVecを用意するのとかも含めてかなりの計算量になりそう
// pub struct PassiveEffecter<'a> {
//     inner: &'a mut VecDeque<UpdateStateMessage>,
//     begined: bool,
//     last_item_exec_priority: u8,
// }

// impl<'a> PassiveEffecter<'a> {
//     /// 誘発の無限ループを防ぐためにある
//     /// 無限ループを防ぐ仕組みを実装したとしても、そういうゲーム性は
//     /// 面白くないと思うから気をつける
//     ///
//     /// 全ての誘発能力は1ターンに回数制限があるものとする
//     ///
//     /// そう仮定した場合、全ての誘発可能性のあるイベントより先に回数カ
//     /// ウントを増やすイベントを実行すれば無限ループにはならないはず。
//     ///
//     /// そのための実行優先度
//     ///
//     /// 値が小さいほど先に実行すべき
//     fn exec_priority(msg: &UpdateStateMessage) -> u8 {
//         match msg {
//             UpdateStateMessage::UpdatePassiveState(_, _, _) => 1,
//             _ => 2,
//         }
//     }

//     fn new(buffer: &'a mut VecDeque<UpdateStateMessage>) -> Self {
//         Self {
//             inner: buffer,
//             begined: false,
//             last_item_exec_priority: 0,
//         }
//     }

//     pub(crate) fn begin(&mut self) {
//         assert!(!self.begined);
//         self.begined = true;
//         self.last_item_exec_priority = 0;
//     }

//     pub(crate) fn end(&mut self) {
//         assert!(self.begined);
//         self.begined = false;
//     }

//     fn inner_push(&mut self, msg: UpdateStateMessage) {
//         assert!(self.begined);
//         let priority = Self::exec_priority(&msg);
//         assert!(
//             priority >= self.last_item_exec_priority,
//             "{}の{}::exec_priorityを確認してください",
//             file!(),
//             type_name::<PassiveEffecter>()
//         );
//         self.last_item_exec_priority = priority;
//         self.inner.push_back(msg);
//     }

//     pub(crate) fn ended(&self) -> bool {
//         !self.begined
//     }
// }

// //
// // 変更する際はAIに任せるのが良さそう
// // マクロの作成も視野に入れてもいい
// //
// impl<'a> PassiveEffecter<'a> {
//     pub fn dmg(&mut self, dmg: damage::Damage) {
//         let msg = UpdateStateMessage::Damage(dmg);
//         self.inner_push(msg);
//     }

//     pub fn heal(&mut self, target: LtId, num: StatusNum) {
//         let msg = UpdateStateMessage::HealHp(target, num);
//         self.inner_push(msg);
//     }

//     pub fn consume_mp(&mut self, num: MpNum) {
//         let msg = UpdateStateMessage::ConsumeMp(num);
//         self.inner_push(msg);
//     }

//     pub fn heal_mp(&mut self, num: MpNum) {
//         let msg = UpdateStateMessage::HealMp(num);
//         self.inner_push(msg);
//     }

//     pub fn consume_sp(&mut self, target: RuntimeEnemyId, num: SpNum) {
//         let msg = UpdateStateMessage::ConsumeSp(target, num);
//         self.inner_push(msg);
//     }

//     pub fn heal_sp(&mut self, target: RuntimeEnemyId, num: SpNum) {
//         let msg = UpdateStateMessage::HealSp(target, num);
//         self.inner_push(msg);
//     }

//     pub fn update_passive_state(
//         &mut self,
//         target: LtId,
//         passive_id: StaticPassiveId,
//         message: PassiveUpdateStateMessage,
//     ) {
//         let msg = UpdateStateMessage::UpdatePassiveState(target, passive_id, message);
//         self.inner_push(msg);
//     }

//     pub fn add_passive(&mut self, target: LtId, passive: Box<dyn Passive>) {
//         let msg = UpdateStateMessage::AddPassive(target, passive);
//         self.inner_push(msg);
//     }

//     pub fn set_skill_cooldown(
//         &mut self,
//         char_id: RuntimeCharId,
//         skill_id: RuntimeSkillId,
//         cooldown: CooldownNum,
//     ) {
//         let msg = UpdateStateMessage::SetSkillCooldown(char_id, skill_id, cooldown);
//         self.inner_push(msg);
//     }

//     pub fn heal_skill_cooldown(
//         &mut self,
//         char_id: RuntimeCharId,
//         skill_id: RuntimeSkillId,
//         cooldown: CooldownNum,
//     ) {
//         let msg = UpdateStateMessage::HealSkillCooldown(char_id, skill_id, cooldown);
//         self.inner_push(msg);
//     }

//     pub fn heal_skill_cooldown_all(&mut self, char_id: RuntimeCharId, cooldown: CooldownNum) {
//         let msg = UpdateStateMessage::HealSkillCooldownAll(char_id, cooldown);
//         self.inner_push(msg);
//     }

//     pub fn add_hate(&mut self, char_id: RuntimeCharId, hate: HateNum) {
//         let msg = UpdateStateMessage::AddHate(char_id, hate);
//         self.inner_push(msg);
//     }
// }
