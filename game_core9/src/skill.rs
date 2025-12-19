use std::{
    collections::VecDeque,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use downcast_rs::{Downcast, impl_downcast};
use dyn_clone::DynClone;

use crate::{
    StaticSkillId, StatusNum, TimeNum, any_message::AnyMessageBox, core_actor::EffectsBuffer, effect::Effect, game_state::GameState, progress_state::ProgressState, runtime_id::{RuntimeCharId, RuntimeSkillId}
};

//--------------------------------------------------//
//                                                  //
//                    SKILL BOX                     //
//                                                  //
//--------------------------------------------------//

#[derive(Debug, Clone)]
pub struct SkillBox(Box<dyn SkillTrait>);
impl SkillBox {
    pub fn new(inner: impl SkillTrait + 'static) -> Self {
        Self(Box::new(inner))
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

//--------------------------------------------------//
//                                                  //
//                       INFO                       //
//                                                  //
//--------------------------------------------------//
pub struct SkillInfomation {
    pub name: &'static str,
    pub description: &'static str,
    pub flaver_text: &'static str,
    pub id: StaticSkillId,
    pub default_need_mp: StatusNum,
    pub defalut_hate: StatusNum,
    pub defalut_cooldown: TimeNum,
}

pub trait SkillTrait: Debug + Downcast + DynClone {
    fn current_progress(&self) -> Option<CharSkillProgress>;

    fn tick(
        &self,
        owner_id: RuntimeSkillId,
        state: &GameState,
        effects_buffer: &mut EffectsBuffer,
    );

    fn start(&mut self);
    fn update(&mut self, msg: &AnyMessageBox);

    fn info(&self) -> &SkillInfomation;

    #[allow(unused_variables)]
    fn need_mp(&self, self_id: RuntimeSkillId, state: &GameState) -> StatusNum {
        self.info().default_need_mp
    }

    #[allow(unused_variables)]
    fn custom_useable(&self, self_id: RuntimeSkillId, state: &GameState) -> SkillCustomUseable {
        SkillCustomUseable::Normal
    }
}
impl_downcast!(SkillTrait);
dyn_clone::clone_trait_object!(SkillTrait);

#[derive(Debug, Clone)]
pub enum SkillCustomUseable {
    /// need_mp, cooldownなど全ての他の要因を無視してこの値を適用する
    Strong(bool),
    /// MPが足りない場合でも使用可能。クールダウンは無視しない。
    IgnoreNeedMp,
    /// クールダウン中でも使用可能。MPは無視しない。
    IgnoreCooldown,
    /// 特になし。need_mpとcooldownに依存する
    Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharSkillProgressKind {
    Chanting,
    Acting,
}

#[derive(Debug, Clone)]
pub struct CharSkillProgress {
    pub kind: CharSkillProgressKind,
    /// 全体フレームに対しての進捗ではないことに注意
    /// kindがChantingなら"詠唱"の進捗
    pub chunk_progress: ProgressState,
    /// 全体フレームに対しての進捗
    pub overall_progress: ProgressState,
}
