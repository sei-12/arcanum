use std::sync::Arc;

use bebytes::BeBytes;
use game_core7::{
    StatusNum,
    any_message::AnyMessage,
    effect::Effect,
    skill::{SkillBox, SkillTrait},
};

#[derive(Debug, Clone)]
enum FireBallMessage {
    Tick { char_speed: StatusNum },
}

#[derive(Debug, Clone)]
pub struct FireBall {
    check_num: u8,
    time: StatusNum,
}
impl FireBall {
    pub fn new() -> Self {
        Self {
            check_num: 0,
            time: 0.0,
        }
    }
}

impl Default for FireBall {
    fn default() -> Self {
        Self::new()
    }
}
impl SkillTrait for FireBall {
    fn clone_instance(&self) -> SkillBox {
        SkillBox::new(self.clone())
    }

    fn start(&mut self) {
        self.check_num = 0;
        self.time = 0.0;
    }

    fn current_condition(
        &self,
        current_skill_state: &game_core7::skill::UsingSkillState,
    ) -> game_core7::buttle_char::ButtleCharCondition {
        todo!()
    }

    fn frame(
        &self,
        owner: game_core7::runtime_id::RuntimeCharId,
        state: &game_core7::state::GameState,
        current_skill_state: &game_core7::skill::UsingSkillState,
        ctx: &mut game_core7::core_actor::CtxContainer,
    ) {
        let speed = state.get_char(owner).lt().speed();
        // TODO かなりパフォーマンスが悪い
        // メッセージの方法から見直す必要がある
        // フレーム毎にヒープを確保している
        // SmallBoxにするべきだと思う
        ctx.effects_buffer.push_back(Effect::UpdateSkillState {
            target_id: owner,
            skill_id: current_skill_state.runtime_skill_id,
            msg: AnyMessage::ArcAny(Arc::new(FireBallMessage::Tick { char_speed: speed })),
        });
    }

    fn update(&mut self, msg: &AnyMessage) {
        let m = msg.try_downcast_ref::<FireBallMessage>().unwrap();
        match m {
            FireBallMessage::Tick { char_speed } => self.time += char_speed,
        }
    }

    fn info(&self) -> &game_core7::skill::SkillInfomation {
        &game_core7::skill::SkillInfomation {
            name: "ファイヤーボール",
            description: "",
            id: 1,
            default_need_mp: 10.0,
            defalut_hate: 10.0,
            defalut_cooldown: 10.0,
        }
    }
}
