use std::{
    any::Any,
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use smallbox::{SmallBox, smallbox, space};

use crate::{
    CooldownNum, HateNum, MpNum, StaticSkillId, WinOrLoseOrNextwave,
    effector::EffectorTrait,
    runtime_id::{RuntimeCharId, RuntimeEnemyId, RuntimeSkillId},
    state::GameState,
};

#[derive(Debug, Clone)]
pub enum SkillUpdateMessage {
    Msg(&'static str),
    Buffer([u8; 16]),
    Box(Arc<dyn Any>),
}

#[derive(Debug)]
pub struct SkillInstance(SmallBox<dyn SkillTrait, space::S1>);

impl SkillInstance {
    pub fn new(inner: impl SkillTrait + 'static) -> Self {
        Self(smallbox!(inner))
    }
}

impl Deref for SkillInstance {
    type Target = dyn SkillTrait;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl DerefMut for SkillInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

impl Clone for SkillInstance {
    fn clone(&self) -> Self {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct SkillInfomation {
    pub name: &'static str,
    pub description: &'static str,
    pub id: StaticSkillId,
    pub default_need_mp: MpNum,
    pub defalut_hate: HateNum,
    pub defalut_cooldown: CooldownNum,
}

#[derive(Debug, Clone)]
pub struct SkillCost {
    pub mp: MpNum,
    pub hate: HateNum,
    pub cooldown: CooldownNum,
}

impl SkillCost {
    pub fn from_defalut(info: &SkillInfomation) -> Self {
        Self {
            mp: info.default_need_mp,
            hate: info.defalut_hate,
            cooldown: info.defalut_cooldown,
        }
    }
}

// todo rename: Staticではない
pub trait SkillTrait: Debug {
    fn call(
        &self,
        user_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
        target_id: Option<RuntimeEnemyId>,
        effector: &mut dyn EffectorTrait,
    ) -> Result<SkillCost, WinOrLoseOrNextwave>;
    fn info(&self) -> &SkillInfomation;
    fn clone(&self) -> SkillInstance;

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
    fn update(&mut self, msg: &SkillUpdateMessage) {
        unimplemented!()
        // ほとんどのスキルには必要ないメソッドなのでunimplemented
    }
}
