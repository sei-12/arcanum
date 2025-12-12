use std::{
    collections::VecDeque, fmt::Debug, ops::{Deref, DerefMut}
};

use crate::{
    CooldownNum, HateNum, MpNum, StaticSkillId, TimeNum,
    any_message::AnyMessage,
    buttle_char::{ButtleChar, ButtleCharCondition},
    effect::Effect,
    runtime_id::RuntimeCharId,
    state::GameState,
};

#[derive(Debug, Clone)]
pub struct SkillInfomation {
    pub name: &'static str,
    pub description: &'static str,
    pub id: StaticSkillId,
    pub default_need_mp: MpNum,
    pub defalut_hate: HateNum,
    pub defalut_cooldown: CooldownNum,
}

pub trait SkillTrait: Debug {
    fn info(&self) -> &SkillInfomation;
    fn clone_instance(&self) -> SkillBox;

    fn frame(
        &self,
        owner: RuntimeCharId,
        state: &GameState,
        current_skill_state: &UsingSkillState,
        effects_buffer: &mut VecDeque<Effect>,
    );

    fn current_condition(&self, current_skill_state: &UsingSkillState) -> ButtleCharCondition;

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

#[derive(Debug, Clone)]
pub struct UsingSkillState {
    /// スキル使用時フレームが0
    frame: u32,
    /// スキル開始時に0
    /// 毎フレームキャラクターのAGIに応じて加算される値
    time: TimeNum,
}

impl UsingSkillState {
    pub fn new() -> Self {
        Self {
            frame: 0,
            time: 0.0,
        }
    }
}

impl Default for UsingSkillState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct SkillBox(Box<dyn SkillTrait>);

impl SkillBox {
    pub fn new(skill: impl SkillTrait + 'static) -> Self {
        Self(Box::new(skill))
    }
}

impl Deref for SkillBox {
    type Target = dyn SkillTrait;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl DerefMut for SkillBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl Clone for SkillBox {
    fn clone(&self) -> Self {
        self.0.clone_instance()
    }
}
