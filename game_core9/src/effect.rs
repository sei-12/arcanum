use crate::{
    StaticPassiveId, StatusNum, TimeNum,
    any_message::MessageBox,
    damage,
    passive::PassiveBox,
    runtime_id::{LtId, RuntimeCharId, RuntimeSkillId},
};

#[derive(Debug, Clone)]
pub enum Effect {
    Damage(damage::Damage),
    HealHp {
        target_id: LtId,
        num: StatusNum,
    },
    ConsumeMp {
        target_id: LtId,
        num: StatusNum,
    },
    HealMp {
        target_id: LtId,
        num: StatusNum,
    },
    UpdatePassiveState {
        target_id: LtId,
        passive_id: StaticPassiveId,
        message: MessageBox,
    },
    AddPassive {
        target_id: LtId,
        passive: PassiveBox,
    },
    AddSkillCooldown {
        target_id: RuntimeSkillId,
        num: TimeNum,
    },
    HealSkillCooldownAll {
        target_id: RuntimeCharId,
        num: TimeNum,
    },
    AddHate {
        target_id: RuntimeCharId,
        num: StatusNum,
    },
    UpdateSkillState {
        skill_id: RuntimeSkillId,
        msg: MessageBox,
    },
}
