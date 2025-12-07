use std::{
    any::Any,
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use smallbox::{SmallBox, smallbox, space};

use crate::{
    StaticSkillId, WinOrLoseOrNextwave, effector::EffectorTrait, runtime_id::RuntimeCharId,
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

// todo rename: Staticではない
pub trait SkillTrait: Debug {
    fn static_id(&self) -> StaticSkillId;
    fn call(
        &self,
        user: RuntimeCharId,
        effector: &mut dyn EffectorTrait,
    ) -> Result<(), WinOrLoseOrNextwave>;
    fn clone(&self) -> SkillInstance;
    fn update(&mut self, msg: &SkillUpdateMessage);
}
