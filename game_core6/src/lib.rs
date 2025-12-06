#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod damage;
pub mod effect;
pub mod effector;
pub mod living_thing;
pub mod output;
pub mod state;

use std::{any::TypeId, collections::VecDeque};

use crate::{output::GameCoreOutput, state::GameState};

pub type MpNum = u32;
pub type SpNum = u32;
pub type LevelNum = u32;
pub type StatusNum = f32;
pub type TurnNum = u8;
pub type CooldownNum = u32;
pub type HateNum = u32;
pub type WaveNum = u8;
pub type StaticEnemySkillId = u32;
pub type StaticSkillId = u32;
pub type StaticPassiveId = TypeId;
pub type StaticCharId = u32;
pub type StaticEnemyId = u32;

pub mod runtime_id {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct RuntimeCharId {
        pub(crate) idx: u8,
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct RuntimeEnemyId {
        pub(crate) wave_idx: u8,
        pub(crate) idx: u8,
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct RuntimeSkillId {
        pub(crate) char_id: RuntimeCharId,
        pub(crate) idx: u8,
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub enum LtId {
        Char(RuntimeCharId),
        Enemy(RuntimeEnemyId),
    }
}

pub mod game_core_actor {
    use std::collections::VecDeque;

    use crate::{
        output::GameCoreOutput, receiver_side::ReceiverSide, sender_side::SenderSide,
        state::GameState,
    };

    pub enum GaemCoreActorCommand {}

    pub struct GameCoreActor {
        sender_side: SenderSide,
        output_bufffer: VecDeque<GameCoreOutput>,
        receiver_side: ReceiverSide,
    }

    impl GameCoreActor {
        pub fn send_cmd(&mut self, cmd: GaemCoreActorCommand) {
            todo!()
        }

        pub fn forward(&mut self) -> Option<GameCoreOutput> {
            self.receiver_side.forward(&mut self.output_bufffer)
        }

        pub fn state(&self) -> &GameState {
            self.receiver_side.state()
        }
    }
}

trait OutputBuffer {
    fn push(&mut self, item: GameCoreOutput);
    fn pop(&mut self) -> Option<GameCoreOutput>;
}
impl OutputBuffer for VecDeque<GameCoreOutput> {
    fn push(&mut self, item: GameCoreOutput) {
        self.push_back(item);
    }
    fn pop(&mut self) -> Option<GameCoreOutput> {
        self.pop_front()
    }
}

mod skill {
    use std::{
        any::Any,
        ops::{Deref, DerefMut},
    };

    use smallbox::{
        SmallBox, smallbox,
        space::{self, S2},
    };

    use crate::{StaticSkillId, effector::EffectorTrait, runtime_id::RuntimeCharId};

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
        fn call(&self, user: RuntimeCharId, effector: &mut dyn EffectorTrait);
        fn clone(&self) -> SkillInstance;
        fn update(&mut self, msg: &SkillUpdateMessage);
    }
}

pub mod enemy {
    use std::ops::{Deref, DerefMut};

    use smallbox::{SmallBox, smallbox, space};

    use crate::{
        StaticEnemyId, StaticEnemySkillId, effector::EffectorTrait, living_thing::Potential,
        runtime_id::RuntimeEnemyId, state::GameState,
    };

    //--------------------------------------------------//
    //                   ENEMY SKILL                    //
    //--------------------------------------------------//
    pub struct EnemySkillInsance(SmallBox<dyn StaticEnemySkillData, space::S1>);
    impl EnemySkillInsance {
        pub fn new(skill_data: impl StaticEnemySkillData + 'static) -> Self {
            Self(smallbox!(skill_data))
        }
    }

    pub trait StaticEnemySkillData {
        fn static_id(&self) -> StaticEnemySkillId;
        fn call(&self, user_id: RuntimeEnemyId, effector: &mut dyn EffectorTrait);
        fn clone(&self) -> EnemySkillInsance;
    }

    impl Deref for EnemySkillInsance {
        type Target = dyn StaticEnemySkillData;
        fn deref(&self) -> &Self::Target {
            self.0.deref()
        }
    }
    impl DerefMut for EnemySkillInsance {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.0.deref_mut()
        }
    }
    impl Clone for EnemySkillInsance {
        fn clone(&self) -> Self {
            self.0.clone()
        }
    }

    //--------------------------------------------------//
    //                STATIC ENEMY DATA                 //
    //--------------------------------------------------//
    pub struct StaticEnemyDataInstance(SmallBox<dyn StaticEnemyData, space::S1>);
    impl StaticEnemyDataInstance {
        pub fn new(enemy_data: impl StaticEnemyData + 'static) -> Self {
            Self(smallbox!(enemy_data))
        }
    }

    impl Deref for StaticEnemyDataInstance {
        type Target = dyn StaticEnemyData;
        fn deref(&self) -> &Self::Target {
            self.0.deref()
        }
    }
    impl DerefMut for StaticEnemyDataInstance {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.0.deref_mut()
        }
    }
    impl Clone for StaticEnemyDataInstance {
        fn clone(&self) -> Self {
            self.0.clone()
        }
    }

    pub trait StaticEnemyData {
        fn static_id(&self) -> StaticEnemyId;
        fn select_skill(&self, user_id: RuntimeEnemyId, state: &GameState) -> EnemySkillInsance;
        fn potential(&self) -> &Potential;
        fn clone(&self) -> StaticEnemyDataInstance;
    }
}

mod passive;

mod sender_side {

    use std::collections::VecDeque;

    use crate::{
        OutputBuffer,
        effector::{Effector, EffectorTrait},
        output::GameCoreOutput,
        runtime_id::{RuntimeCharId, RuntimeSkillId},
        state::GameState,
    };
    pub(crate) struct SenderSide {
        state: GameState,
    }
    impl SenderSide {
        pub(crate) fn game_start(&mut self, output_buffer: &mut impl OutputBuffer) {}
        pub(crate) fn use_skill(
            &mut self,
            user_id: RuntimeCharId,
            skill_id: RuntimeSkillId,
            output_buffer: &mut impl OutputBuffer,
        ) {
            let char = self.state.get_char(user_id);
            let skill = char.get_skill(skill_id).clone();
            let char_runtime_id = char.runtime_id();
            let mut effector = Effector::new(&mut self.state, output_buffer);
            effector.begin_char_skill(skill.static_id(), user_id);
            skill.call(char_runtime_id, &mut effector);
        }

        pub(crate) fn trun_end(&mut self, output_buffer: &mut impl OutputBuffer) {
            let mut effector = Effector::new(&mut self.state, output_buffer);
            effector.start_enemy_turn();

            effector.begin_game_system();
            effector.end_game_system();


            let mut enemys = effector.state().enemys_with_living_check();
            while let Some(enemy) = enemys.next_livint_enemy(effector.state()) {
                let enemy_skill = enemy
                    .static_data()
                    .select_skill(enemy.runtime_id(), effector.state());

                enemy_skill.call(enemy.runtime_id(), &mut effector);
            }

            effector.start_player_turn();
        }
    }
}

mod receiver_side {
    use crate::{OutputBuffer, output::GameCoreOutput, state::GameState};
    use std::collections::VecDeque;

    pub(crate) struct ReceiverSide {
        state: GameState,
    }

    impl ReceiverSide {
        pub(crate) fn forward(
            &mut self,
            output_buffer: &mut impl OutputBuffer,
        ) -> Option<GameCoreOutput> {
            todo!()
        }

        pub(crate) fn state(&self) -> &GameState {
            &self.state
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WinOrLoseOrNextwave {
    Win,
    Lose,
    Nextwave,
}

impl WinOrLoseOrNextwave {
    fn is_win_or_lose(&self) -> bool {
        *self == WinOrLoseOrNextwave::Win || *self == WinOrLoseOrNextwave::Lose
    }
}
