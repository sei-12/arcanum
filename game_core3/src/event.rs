use std::cell::RefMut;

use crate::{
    CooldownNum, GameResult, HateNum, MpNum, SpNum,
    damage::Damage,
    enemys::RuntimeEnemyId,
    passive::{PassiveUpdateStateMessage, RuntimePassiveId, traits::Passive},
    skill::StaticSkillId,
    state::{LtId, Side, chars::RuntimeCharId},
};

pub trait EventsQuePusher {
    fn push_event(&mut self, event: Event);
}

impl EventsQuePusher for Vec<Event> {
    fn push_event(&mut self, event: Event) {
        self.push(event);
    }
}
impl<'a> EventsQuePusher for RefMut<'a, Vec<Event>> {
    fn push_event(&mut self, event: Event) {
        self.push(event);
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Log(String),
    Damage(Damage),
    TurnStart(Side),
    HealMp {
        mp: MpNum,
    },
    AddPassive {
        target_id: LtId,
        passive: Box<dyn Passive>,
    },
    UpdatePassiveState {
        target_id: LtId,
        passive_id: RuntimePassiveId,
        msg: PassiveUpdateStateMessage,
    },
    GameEnd(GameResult),
    ConsumeMp {
        mp: MpNum,
    },
    AddHate {
        char_id: RuntimeCharId,
        hate: HateNum,
    },
    SetSkillCooldown {
        char_id: RuntimeCharId,
        skill_id: StaticSkillId,
        cooldown: CooldownNum,
    },
    HealSkillCooldown {
        char_id: RuntimeCharId,
        skill_id: StaticSkillId,
        heal_num: CooldownNum,
    },
    HeallSkillCooldownAll {
        char_id: RuntimeCharId,
        heal_num: CooldownNum,
    },
    ConsumeSp {
        enemy_id: RuntimeEnemyId,
        num: SpNum,
    },
    HealSp {
        enemy_id: RuntimeEnemyId,
        num: SpNum,
    },
    GoNextWave,
    UnFocusEnemy,
    ChangeFocusEnemy {
        enemy_id: RuntimeEnemyId,
    },
    UseSkill {
        user_name: &'static str,
        skill_name: &'static str,
    },
}
