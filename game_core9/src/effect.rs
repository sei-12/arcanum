use crate::{
    StaticPassiveId, StatusNum, TimeNum,
    any_message::AnyMessageBox,
    damage,
    passive::PassiveBox,
    runtime_id::{LtId, RuntimeCharId, RuntimeSkillId},
};

#[derive(Debug, Clone)]
pub enum Effect {
    EnemySkillRunnerIncrementFrame,
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
        message: AnyMessageBox,
    },
    AddPassive {
        target_id: LtId,
        passive: PassiveBox,
    },
    AddSkillCooldown {
        skill_id: RuntimeSkillId,
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
        msg: AnyMessageBox,
    },
    UseSkill {
        skill_id: RuntimeSkillId,
    },
}
