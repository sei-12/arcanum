use std::any::Any;

use crate::{
    TurnNum,
    damage::Damage,
    event::Event,
    passive::{
        DisplayPassiveInfo, PassiveUpdateStateError, PassiveUpdateStateMessage, RuntimePassiveId,
        gen_passive_runtime_id, status::PassiveStatus, traits::Passive,
    },
};

#[derive(Debug, Clone)]
pub struct Burn {
    id: RuntimePassiveId,
    turns: TurnNum,
    header: String,
}

impl Burn {
    fn header(turns: TurnNum) -> String {
        format!("火傷({})", turns)
    }

    pub fn new(turns: TurnNum) -> Self {
        Self {
            id: gen_passive_runtime_id(),
            turns,
            header: Self::header(turns),
        }
    }
}

impl Passive for Burn {
    fn display(&'_ self) -> Option<crate::passive::DisplayPassiveInfo<'_>> {
        Some(DisplayPassiveInfo {
            header: std::borrow::Cow::Borrowed(&self.header),
            text: "火傷
            ・被魔法ダメージ1.1倍
            ・ターン開始時に残りHPの3%の固定ダメージを受ける。VITが7以下なら代わりに7%の固定ダメージを受ける。
            ".into(),
        })
    }

    fn runtime_id(&self) -> crate::passive::RuntimePassiveId {
        self.id
    }

    fn static_id(&self) -> std::any::TypeId {
        self.type_id()
    }

    fn should_trash(&self) -> bool {
        self.turns == 0
    }

    fn status(&self, status: &mut PassiveStatus) {
        assert!(!self.should_trash());

        status.recv_magic_dmg_mag.mul(1.1);
    }

    fn should_merge_type(&self) -> bool {
        true
    }

    fn merge_state(&self, buffer: &mut [u8]) {
        assert!(buffer.len() == 1);
        buffer[0] = self.turns;
    }

    fn merge(&mut self, passive: Box<dyn Passive>) {
        assert_eq!(self.static_id(), passive.static_id());
        let mut buffer = [0];
        passive.merge_state(&mut buffer);
        self.turns = self.turns.saturating_add(buffer[0]);
        self.header = Self::header(self.turns);
    }

    fn trigger_turn_start(
        &self,
        owner_id: crate::state::LtId,
        state: &crate::state::GameState,
        effects: &mut Vec<crate::event::Event>,
    ) {
        assert!(!self.should_trash());
        let owner = state.get_lt(owner_id);

        let mut dmg_per = 0.03;
        if owner.vit() <= 7.0 {
            dmg_per = 0.07;
        };

        let dmg = Event::Damage(Damage::new_hp_per_dmg(state, owner_id, dmg_per));

        effects.push(dmg);

        effects.push(Event::UpdatePassiveState {
            target_id: owner_id,
            passive_id: self.runtime_id(),
            msg: PassiveUpdateStateMessage::DecrimentTurns,
        });
    }

    fn update_state(
        &mut self,
        msg: &crate::passive::PassiveUpdateStateMessage,
    ) -> Result<(), PassiveUpdateStateError> {
        assert!(!self.should_trash());

        match &msg {
            PassiveUpdateStateMessage::DecrimentTurns => {
                self.turns -= 1;
            }
            _ => {
                return Err(PassiveUpdateStateError::UnexpectedMessage(msg.clone()));
            }
        }
        Ok(())
    }
}
