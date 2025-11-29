use crate::{
    CooldownNum, GameResult, HateNum, MpNum,
    damage::Damage,
    enemys::RuntimeEnemyId,
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
    GoNextWave,
    UnFocusEnemy,
    ChangeFocusEnemy {
        enemy_id: RuntimeEnemyId,
    },
    DeadEnemy {
        enemy_id: RuntimeEnemyId,
    },
    UseSkill {
        user_name: &'static str,
        skill_name: &'static str,
    },
}
