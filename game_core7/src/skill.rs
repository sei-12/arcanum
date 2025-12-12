use std::collections::VecDeque;

use crate::{
    CooldownNum, HateNum, MpNum, StaticSkillId, TimeNum, any_message::AnyMessage,
    buttle_char::ButtleChar, effect::Effect, runtime_id::RuntimeCharId, state::GameState,
};

type FrameFn = fn(&ButtleChar, &GameState, &mut VecDeque<Effect>);
type NeedMpFn = fn(&ButtleChar, &GameState) -> MpNum;
type CustomUseableFn = fn(&ButtleChar, &GameState) -> bool;

#[derive(Debug, Clone)]
pub struct SkillInfomation {
    pub name: &'static str,
    pub description: &'static str,
    pub id: StaticSkillId,
    pub default_need_mp: MpNum,
    pub defalut_hate: HateNum,
    pub defalut_cooldown: CooldownNum,
}

pub trait StaticSkillData {
    fn info(&self) -> &SkillInfomation;

    fn frame(&self, owner: RuntimeCharId, state: &GameState, effects_buffer: &mut VecDeque<Effect>);

    #[allow(unused_variables)]
    /// 特殊な条件が効果にない場合、self.doc().default_need_mpを返す
    fn need_mp(&self, owner: RuntimeCharId, state: &GameState) -> MpNum {
        self.info().default_need_mp
    }

    #[allow(unused_variables)]
    /// mpとcooldown以外の要因で変わる場合は値を返す
    /// Someを返す場合、mpとcooldownなどの要因を全て無視して返された値を適用する
    fn custom_useable(&self, owner: RuntimeCharId, state: &GameState) -> Option<bool> {
        None
    }

    #[allow(unused_variables)]
    fn update(&mut self, msg: &AnyMessage) {
        unimplemented!()
        // ほとんどのスキルには必要ないメソッドなのでunimplemented
    }
}

pub struct UsingSkillState {
    /// スキル使用時フレームが0
    frame: u32,
    /// スキル開始時に0
    /// 毎フレームキャラクターのAGIに応じて加算される値
    time: TimeNum,
}

// pub struct StaticSkillDataBuilder {
//     name: Option<&'static str>,
//     desctiption: Option<&'static str>,
//     frame_fn: Option<FrameFn>,
//     default_need_mp:
// }

// impl StaticSkillDataBuilder {
//     pub const fn new() -> Self {
//         Self {
//             desctiption: None,
//             frame_fn: None,
//             name: None,
//         }
//     }

//     pub const fn name(mut self, name: &'static str) -> Self {
//         self.name = Some(name);
//         self
//     }

//     pub const fn build(self) -> StaticSkillData {
//         StaticSkillData {
//             desctiption: self.desctiption.unwrap(),
//             name: self.name.unwrap(),
//             frame_fn: self.frame_fn.unwrap(),
//         }
//     }
// }

// impl Default for StaticSkillDataBuilder {
//     fn default() -> Self {
//         Self::new()
//     }
// }
