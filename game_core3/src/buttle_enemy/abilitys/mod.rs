use std::any::Any;

use crate::{
    damage::DamageType,
    event::Event,
    passive::{
        DisplayPassiveInfo, PassiveUpdateStateError, PassiveUpdateStateMessage, RuntimePassiveId,
        gen_passive_runtime_id, traits::Passive,
    },
    state::LtId,
};

pub enum EnemyAbility {
    // 変質
    Transition,
    // 逆襲
    Revenge,
}

pub(crate) fn make_enemy_ability(ability: EnemyAbility) -> Box<dyn Passive> {
    match ability {
        EnemyAbility::Revenge => Box::new(Revenge::new()),
        _ => {
            todo!()
        }
    }
}

//--------------------------------------------------//
//                                                  //
//                     REVENGE                      //
//                                                  //
//--------------------------------------------------//

#[derive(Debug, Clone)]
struct Revenge {
    id: RuntimePassiveId,
    count: u8,
    header: String,
}
impl Revenge {
    fn new() -> Self {
        Self {
            id: gen_passive_runtime_id(),
            count: 0,
            header: "逆襲".to_string(),
        }
    }

    fn update_header(&mut self) {
        if self.count == 0 {
            self.header = "逆集".to_string()
        } else {
            self.header = format!("逆襲 ({})", self.count);
        }
    }
}

impl Passive for Revenge {
    fn display(&'_ self) -> Option<crate::passive::DisplayPassiveInfo<'_>> {
        Some(crate::passive::DisplayPassiveInfo {
            header: std::borrow::Cow::Borrowed(&self.header),
            text: "逆襲
            ・キャラクターからダメージを受けるたびにSPを10回復し、攻撃キャラクターのヘイト値を+5する。この効果は1ターンに10度まで誘発する。
            ".into(),
        })
    }

    fn merge(&mut self, passive: Box<dyn Passive>) {
        assert_eq!(self.static_id(), passive.static_id());
    }

    fn runtime_id(&self) -> crate::passive::RuntimePassiveId {
        self.id
    }
    fn should_merge_type(&self) -> bool {
        false
    }
    fn should_trash(&self) -> bool {
        false
    }

    fn static_id(&self) -> std::any::TypeId {
        self.type_id()
    }

    fn status(&self, _status: &mut crate::passive::status::PassiveStatus) {}

    fn update_state(
        &mut self,
        msg: &crate::passive::PassiveUpdateStateMessage,
    ) -> Result<(), crate::passive::PassiveUpdateStateError> {
        match msg {
            PassiveUpdateStateMessage::DecrimentTurns => {
                self.count = 0;
            }
            PassiveUpdateStateMessage::Unique(_) => {
                self.count += 1;
            }
            _ => {
                return Err(crate::passive::PassiveUpdateStateError::UnexpectedMessage(
                    msg.clone(),
                ));
            }
        }

        self.update_header();

        Ok(())
    }

    fn merge_state(&self, _buffer: &mut [u8]) {
        panic!("mergeできない")
    }

    fn trigger_recv_damage(
        &self,
        owner_id: crate::state::LtId,
        _state: &crate::state::GameState,
        dmg: &crate::damage::Damage,
        effects: &mut Vec<crate::event::Event>,
    ) {
        let LtId::Enemy(owner_enemy_id) = owner_id else {
            panic!("Revengeは敵限定のアビリティ");
        };

        let Some(LtId::Char(causer_char_id)) = dmg.causer().to_lt_id() else {
            return;
        };

        effects.push(Event::HealSp {
            enemy_id: owner_enemy_id,
            num: 10,
        });

        effects.push(Event::AddHate {
            char_id: causer_char_id,
            hate: 5,
        });

        effects.push(Event::UpdatePassiveState {
            target_id: owner_id,
            passive_id: self.id,
            msg: crate::passive::PassiveUpdateStateMessage::Unique(0),
        });
    }

    fn trigger_turn_start(
        &self,
        owner_id: LtId,
        _state: &crate::state::GameState,
        effects: &mut Vec<Event>,
    ) {
        effects.push(Event::UpdatePassiveState {
            target_id: owner_id,
            passive_id: self.id,
            msg: crate::passive::PassiveUpdateStateMessage::TriggerTurnStart,
        });
    }
}

//--------------------------------------------------//
//                                                  //
//                    TRANSITION                    //
//                                                  //
//--------------------------------------------------//

#[derive(Debug, Clone)]
struct Transition {
    id: RuntimePassiveId,
    count: i32,
    current_defence_type: Option<DamageType>,
    header: String,
}

impl Transition {
    fn new() -> Self {
        todo!()
    }

    fn update_header(&mut self) {}

    fn update_msg_type_to_u64(ty: DamageType) -> u64 {
        match ty {
            DamageType::Magic => 0,
            DamageType::Physics => 1,
            _ => panic!(),
        }
    }

    fn update_msg_u64_to_type(num: u64) -> Result<DamageType, PassiveUpdateStateError> {
        match num {
            0 => Ok(DamageType::Magic),
            1 => Ok(DamageType::Physics),
            _ => Err(PassiveUpdateStateError::InvalidValue),
        }
    }
}

impl Passive for Transition {
    fn display(&'_ self) -> Option<crate::passive::DisplayPassiveInfo<'_>> {
        Some(DisplayPassiveInfo {
            header: std::borrow::Cow::Borrowed(&self.header),
            text: "変質
            変数: 物理特化,魔法特化
            ・物理ダメージを受けた時、物理特化が1加算され、魔法特化が0になる。
            ・魔法ダメージを受けた時、魔法特化が1加算され、物理特化が0になる。
            ・物理特化1につき被物理ダメージ倍率が0.95倍。被魔法ダメージ倍率が1.025倍。
            ・魔法特化1につき被魔法ダメージ倍率が0.95倍。被物理ダメージ倍率が1.025倍。
            ・補足: 仮に魔法ダメージと物理ダメージを交互に受けた場合、それらすべては1.025倍になる。
            "
            .into(),
        })
    }
    fn merge(&mut self, passive: Box<dyn Passive>) {
        assert_eq!(self.static_id(), passive.static_id());
    }
    fn merge_state(&self, buffer: &mut [u8]) {
        panic!()
    }
    fn runtime_id(&self) -> RuntimePassiveId {
        self.id
    }
    fn should_merge_type(&self) -> bool {
        false
    }
    fn should_trash(&self) -> bool {
        false
    }
    fn static_id(&self) -> std::any::TypeId {
        self.type_id()
    }
    fn status(&self, status: &mut crate::passive::status::PassiveStatus) {
        let Some(current_defence_type) = self.current_defence_type else {
            return;
        };
        let def_mag = 0.95_f32.powi(self.count);
        let weak_mag = 1.025_f32.powi(self.count);

        match current_defence_type {
            DamageType::Magic => {
                status.recv_magic_dmg_mag.mul(def_mag);
                status.recv_physics_dmg_mag.mul(weak_mag);
            }
            DamageType::Physics => {
                status.recv_physics_dmg_mag.mul(def_mag);
                status.recv_magic_dmg_mag.mul(weak_mag);
            }
            _ => {}
        }
    }
    fn update_state(
        &mut self,
        msg: &PassiveUpdateStateMessage,
    ) -> Result<(), crate::passive::PassiveUpdateStateError> {
        match msg {
            PassiveUpdateStateMessage::Unique(num) => {
                let dmg_type = Self::update_msg_u64_to_type(*num)?;
                if Some(dmg_type) == self.current_defence_type {
                    self.count += 1
                } else {
                    self.current_defence_type = Some(dmg_type);
                    self.count = 1;
                }
            }
            _ => {
                return Err(crate::passive::PassiveUpdateStateError::UnexpectedMessage(
                    msg.clone(),
                ));
            }
        }
        self.update_header();
        Ok(())
    }
    fn trigger_recv_damage(
        &self,
        owner_id: LtId,
        _state: &crate::state::GameState,
        dmg: &crate::damage::Damage,
        effects: &mut Vec<Event>,
    ) {
        assert!(matches!(owner_id, LtId::Enemy(_)), "敵限定のアビリティ");
        effects.push(Event::UpdatePassiveState {
            target_id: owner_id,
            passive_id: self.id,
            msg: PassiveUpdateStateMessage::Unique(Self::update_msg_type_to_u64(dmg.ty())),
        });
    }
}
