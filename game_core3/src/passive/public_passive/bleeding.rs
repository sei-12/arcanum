use std::any::Any;

use crate::{
    TurnNum,
    event::Event,
    passive::{
        DisplayPassiveInfo, PassiveUpdateStateError, PassiveUpdateStateMessage, RuntimePassiveId,
        gen_passive_runtime_id, status::PassiveStatus, traits::Passive,
    },
};

#[derive(Debug, Clone)]
pub struct Bleeding {
    id: RuntimePassiveId,
    turns: TurnNum,
    bleed_count: u8,
    header: String,
}

impl Bleeding {
    fn header(turns: TurnNum, count: u8) -> String {
        format!("出血({}, 出血量: {})", turns, count)
    }

    pub fn new(turns: TurnNum) -> Self {
        Self {
            id: gen_passive_runtime_id(),
            turns,
            bleed_count: 0,
            header: Self::header(turns, 0),
        }
    }

    fn add_bleed_count(&mut self, num: u8) {
        self.bleed_count += num;
        if self.bleed_count > 30 {
            self.bleed_count = 30;
        }
    }
}

impl Passive for Bleeding {
    fn display(&'_ self) -> Option<crate::passive::DisplayPassiveInfo<'_>> {
        Some(DisplayPassiveInfo {
            header: std::borrow::Cow::Borrowed(&self.header),
            text: "出血
            ・被物理ダメージ1.05倍
            ・ターン開始時に「変数: 出血量」を2加算する。VITが7以下ならさらに1加算する。
            ・出血量1につき物理攻撃力倍率0.96倍
            ・出血量が20以上ならSTR-1。
            ・出血量の最大値は30
            ・効果は重複しない。重ねがけした場合はターン数と出血量を合算する。
            "
            .into(),
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
        assert!(self.bleed_count <= 30);

        status.recv_physics_dmg_mag.mul(1.05);
        status
            .physics_attuck_mag_debuff
            .mul(0.96_f32.powi(self.bleed_count as i32));

        if self.bleed_count >= 20 {
            status.add_str -= 1.0;
        }
    }

    fn trigger_turn_start(
        &self,
        owner_id: crate::state::LtId,
        state: &crate::state::GameState,
        effects: &mut Vec<crate::event::Event>,
    ) {
        assert!(!self.should_trash());
        let owner = state.get_lt(owner_id);

        let mut add_count = 2;

        if owner.vit() <= 7.0 {
            add_count += 1;
        }

        effects.push(Event::UpdatePassiveState {
            target_id: owner_id,
            passive_id: self.runtime_id(),
            msg: PassiveUpdateStateMessage::DecrimentTurns,
        });

        effects.push(Event::UpdatePassiveState {
            target_id: owner_id,
            passive_id: self.runtime_id(),
            msg: PassiveUpdateStateMessage::UniqueBleeding(add_count),
        });
    }

    fn update_state(
        &mut self,
        msg: &crate::passive::PassiveUpdateStateMessage,
    ) -> Result<(), PassiveUpdateStateError> {
        assert!(!self.should_trash());

        match msg {
            PassiveUpdateStateMessage::DecrimentTurns => {
                self.turns -= 1;
            }
            PassiveUpdateStateMessage::UniqueBleeding(num) => {
                self.add_bleed_count(*num);
            }
            _ => {
                return Err(PassiveUpdateStateError::UnexpectedMessage(msg.clone()));
            }
        }

        self.header = Self::header(self.turns, self.bleed_count);

        Ok(())
    }

    fn merge(&mut self, passive: Box<dyn Passive>) {
        assert_eq!(self.static_id(), passive.static_id());
        let mut buffer = [0, 0];
        passive.merge_state(&mut buffer);
        self.turns = self.turns.saturating_add(buffer[0]);
        self.add_bleed_count(buffer[1]);
        self.header = Self::header(self.turns, self.bleed_count);
    }

    fn merge_state(&self, buffer: &mut [u8]) {
        assert_eq!(buffer.len(), 2);
        buffer[0] = self.turns;
        buffer[1] = self.bleed_count;
    }

    fn should_merge_type(&self) -> bool {
        true
    }
}
