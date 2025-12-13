use crate::{
    CooldownNum, HateNum, MpNum, StaticCharId, StaticPassiveId, StaticSkillId, StatusNum,
    any_message::AnyMessage,
    damage,
    passive::passive_box::PassiveBox,
    runtime_id::{LtId, RuntimeCharId, RuntimeSkillId},
    skill::UsingSkillState,
};

#[derive(Debug, Clone)]
pub enum Effect {
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
    AddSkillCooldown {
        target_id: RuntimeCharId,
        skill_id: StaticSkillId,
        num: CooldownNum,
    },
    HealSkillCooldown {
        target_id: RuntimeCharId,
        skill_id: StaticSkillId,
        num: CooldownNum,
    },
    HealSkillCooldownAll {
        target_id: RuntimeCharId,
        num: CooldownNum,
    },
    HealHate {
        target_id: RuntimeCharId,
        num: HateNum,
    },
    AddHate {
        target_id: RuntimeCharId,
        num: HateNum,
    },
    UpdatePassiveState {
        target: LtId,
        passive_id: StaticPassiveId,
        message: AnyMessage,
    },
    Damage(damage::Damage),
    UseSkill {
        user_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
        state: UsingSkillState,
    },
    AddPassive {
        target_id: LtId,
        passive: PassiveBox,
    },

    UpdateSkillState {
        target_id: RuntimeCharId,
        skill_id: StaticSkillId,
        msg: AnyMessage,
    },
}
