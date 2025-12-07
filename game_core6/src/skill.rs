use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

use smallbox::{
    SmallBox, smallbox,
    space::{self, S2},
};

use crate::{
    StaticSkillId, WinOrLoseOrNextwave,
    effector::EffectorTrait,
    runtime_id::{RuntimeCharId, RuntimeSkillId},
};

pub enum SkillUpdateMessage {
    Msg(&'static str),
    Buffer([u8; 16]),
    Box(Box<dyn Any>),
}

pub struct SkillInstance(SmallBox<dyn StaticSkillData, space::S1>);

impl SkillInstance {
    pub fn new(inner: impl StaticSkillData + 'static) -> Self {
        Self(smallbox!(inner))
    }
}

impl Deref for SkillInstance {
    type Target = dyn StaticSkillData;
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

pub trait StaticSkillData {
    fn static_id(&self) -> StaticSkillId;
    fn call(
        &self,
        user: RuntimeCharId,
        effector: &mut dyn EffectorTrait,
    ) -> Result<(), WinOrLoseOrNextwave>;
    fn clone(&self) -> SkillInstance;
    fn update(&mut self, msg: &SkillUpdateMessage);
}
