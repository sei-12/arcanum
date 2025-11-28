use std::any::Any;

use crate::{
    TurnNum,
    passive::{
        DisplayPassiveInfo, RuntimePassiveId, PassiveUpdateStateError, PassiveUpdateStateMessage,
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

        status.recv_magic_dmg_mag *= 1.1;
    }

    // fn turn_start(
    //     &self,
    //     own_char_idx: crate::game_state::buttle_chars::CharIdx,
    //     state: &crate::game_state::GameState,
    //     effects: &mut Vec<crate::event::EffectEvent>,
    // ) {
    //     assert!(!self.should_trash());

    //     let own_char = state.chars().get_char(own_char_idx);

    //     let mut dmg_per = 0.03;
    //     if own_char.vit() <= 7.0 {
    //         dmg_per = 0.07;
    //     };

    //     let dmg = EffectEvent::Damage(Damage::new_hp_per_dmg(
    //         Target::Char(own_char_idx),
    //         own_char,
    //         dmg_per,
    //     ));

    //     effects.push(dmg);
    //     effects.push(EffectEvent::UpdatePassiveState {
    //         target: Target::Char(own_char_idx),
    //         passive_id: self.id,
    //         msg: crate::passive::PassiveUpdateStateMessage::DecrimentTurns,
    //     });
    // }

    fn update_state(
        &mut self,
        msg: &crate::passive::PassiveUpdateStateMessage,
    ) -> Result<(), PassiveUpdateStateError> {
        assert!(!self.should_trash());

        match msg {
            PassiveUpdateStateMessage::DecrimentTurns => {
                self.turns -= 1;
            }
        }
        Ok(())
    }
}
