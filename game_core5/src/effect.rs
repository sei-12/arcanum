use crate::{
    CooldownNum, HateNum, MpNum, SpNum, StaticPassiveId, StatusNum, damage,
    living_thing::LtId,
    passive::{Passive, PassiveUpdateStateMessage},
    skill::RuntimeSkillId,
    state::{RuntimeCharId, RuntimeEnemyId},
};

#[derive(Debug, Clone)]
pub enum Effect {
    Damage(damage::Damage),
    HealHp {
        target_id: LtId,
        num: StatusNum,
    },
    ConsumeMp {
        num: MpNum,
    },
    HealMp {
        num: MpNum,
    },
    ConsumeSp {
        target_id: RuntimeEnemyId,
        num: SpNum,
    },
    HealSp {
        target_id: RuntimeEnemyId,
        num: SpNum,
    },
    UpdatePassiveState {
        target_id: LtId,
        passive_id: StaticPassiveId,
        message: PassiveUpdateStateMessage,
    },
    AddPassive {
        target_id: LtId,
        passive: Box<dyn Passive>,
    },
    SetSkillCooldown {
        target_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
        num: CooldownNum,
    },
    HealSkillCooldown {
        target_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
        num: CooldownNum,
    },
    HealSkillCooldownAll {
        target_id: RuntimeCharId,
        num: CooldownNum,
    },
    AddHate {
        target_id: RuntimeCharId,
        num: HateNum,
    },
}
