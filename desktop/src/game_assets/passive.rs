use std::any::Any;

use game_core6::{
    StatusNum,
    damage::DamageType,
    effect::Effect,
    passive::{PassiveInstance, PassiveUpdateMessage, StaticPassiveData},
};

#[derive(Debug, Clone)]
pub struct KousitukaPassive {
    turn_count: u8,
    tokka: u16,
}
impl KousitukaPassive {
    const TURN_START: &str = "turn start";
    const RECV_DMG: &str = "recv";
    const RECV_DMG_VIT_13: &str = "recv dmg and vit >= 13";

    pub fn new(turn_count: u8, tokka: u16) -> Self {
        Self { turn_count, tokka }
    }
}
impl StaticPassiveData for KousitukaPassive {
    fn clone_instance(&self) -> game_core6::passive::PassiveInstance {
        PassiveInstance::new(self.clone())
    }
    fn name(&self) -> &'static str {
        "硬質化"
    }
    fn description(&self) -> &'static str {
        "変数: { 特化 , 残りターン数 }
        ・被魔法ダメージ倍率0.95倍
        ・被物理ダメージ倍率0.8倍。さらに特化2につき0.95倍。
        ・物理ダメージを受ける度、特化を2加算する。VITが13以上ならさらに1加算する。
        ・特化40につきAGI-1、VIT+1。ただし、この効果は最大6重まで。
        ・効果が重複する際は特化と残りターン数を合算する。
        "
    }

    fn should_trash(&self) -> bool {
        self.turn_count == 0
    }

    fn static_id(&self) -> game_core6::StaticPassiveId {
        self.type_id()
    }

    fn status(&self, status: &mut game_core6::passive::status::PassiveStatus) {
        status.recv_magic_dmg_mag.mul(0.9);
        let physics_dmg_mag = 0.8 * (0.95_f32.powi((self.tokka / 2) as i32));
        status.recv_physics_dmg_mag.mul(physics_dmg_mag);

        let f = |tokka: u16| u16::min(tokka / 40, 6) as StatusNum;

        let potential = f(self.tokka);

        status.add_agi -= potential;
        status.add_vit += potential;

        debug_assert_eq!(f(1), 0.0);
        debug_assert_eq!(f(40), 1.0);
        debug_assert_eq!(f(41), 1.0);
        debug_assert_eq!(f(239), 5.0);
        debug_assert_eq!(f(240), 6.0);
        debug_assert_eq!(f(300), 6.0);
    }

    fn trigger_turn_start(
        &self,
        owner: game_core6::runtime_id::LtId,
        _state: &game_core6::state::GameState,
        effector: &mut game_core6::effector::PassiveEffector,
    ) {
        effector.accept_effect(Effect::UpdatePassiveState {
            target_id: owner,
            passive_id: self.static_id(),
            message: PassiveUpdateMessage::Msg(Self::TURN_START),
        });
    }

    fn trigger_recv_damage(
        &self,
        owner: game_core6::runtime_id::LtId,
        dmg: &game_core6::damage::Damage,
        state: &game_core6::state::GameState,
        effector: &mut game_core6::effector::PassiveEffector,
    ) {
        if dmg.ty() != DamageType::Physics {
            return;
        }

        let msg = if state.get_lt(owner).vit() >= 13.0 {
            Self::RECV_DMG_VIT_13
        } else {
            Self::RECV_DMG
        };

        effector.accept_effect(Effect::UpdatePassiveState {
            target_id: owner,
            passive_id: self.static_id(),
            message: PassiveUpdateMessage::Msg(msg),
        });
    }

    fn update(&mut self, msg: &game_core6::passive::PassiveUpdateMessage) {
        if msg.is_msg_and(Self::TURN_START) {
            self.turn_count = self.turn_count.saturating_sub(1);
            return;
        }

        if msg.is_msg_and(Self::RECV_DMG) {
            self.tokka = self.tokka.saturating_add(2);
            return;
        }

        if msg.is_msg_and(Self::RECV_DMG_VIT_13) {
            self.tokka = self.tokka.saturating_add(3);
            return;
        }

        unimplemented!()
    }

    fn merge(&mut self, passive: &PassiveInstance) {
        passive.write_merge(self);
    }

    fn write_merge(&self, buffer: &mut dyn Any) {
        let target = buffer.downcast_mut::<Self>().unwrap();
        target.turn_count = target.turn_count.saturating_add(self.turn_count);
        target.tokka = target.tokka.saturating_add(self.tokka);
    }
}
