use crate::{
    CooldownNum, GameResult, HateNum, MpNum,
    damage::Damage,
    passive::{PassiveUpdateStateMessage, RuntimePassiveId, traits::Passive},
    skill::StaticSkillId,
    state::{LtId, Side, chars::RuntimeCharId},
};

pub trait EventsQuePusher {
    fn push(&mut self, event: Event);
}

#[derive(Debug, Clone)]
pub enum Event {
    Log(String),
    Damage(Damage),
    TurnStart(Side),
    HealMp {
        side: Side,
        mp: MpNum,
    },
    AddPassive {
        target: LtId,
        passive: Box<dyn Passive>,
    },
    UpdatePassiveState {
        target: LtId,
        passive_id: RuntimePassiveId,
        msg: PassiveUpdateStateMessage,
    },
    GameEnd(GameResult),
    ConsumeMp {
        side: Side,
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
    ForwordEnemyAction {},
}
