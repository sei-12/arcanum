use crate::{
    CooldownNum, HateNum, MpNum, SpNum, StaticPassiveId, StatusNum, damage,
    passive::{PassiveInstance, PassiveUpdateMessage},
    runtime_id::{LtId, RuntimeCharId, RuntimeEnemyId, RuntimeSkillId},
    skill::SkillUpdateMessage,
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
        message: PassiveUpdateMessage,
    },
    AddPassive {
        target_id: LtId,
        passive: PassiveInstance,
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
    UpdateSkillState {
        target_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
        msg: SkillUpdateMessage,
    },
}

impl Effect {
    /// ターゲットを持っているタイプのエフェクトならターゲットを返す
    pub(crate) fn target(&self) -> Option<LtId> {
        match self {
            Effect::AddHate { target_id, num: _ } => Some(target_id.into()),
            Effect::AddPassive {
                target_id,
                passive: _,
            } => Some(*target_id),
            Effect::ConsumeSp { target_id, num: _ } => Some(target_id.into()),
            Effect::Damage(dmg) => Some(dmg.target()),
            Effect::HealHp { target_id, num: _ } => Some(*target_id),
            Effect::HealSkillCooldown {
                target_id,
                skill_id: _,
                num: _,
            } => Some(target_id.into()),
            Effect::HealSkillCooldownAll { target_id, num: _ } => Some(target_id.into()),
            Effect::HealSp { target_id, num: _ } => Some(target_id.into()),
            Effect::SetSkillCooldown {
                target_id,
                skill_id: _,
                num: _,
            } => Some(target_id.into()),
            Effect::UpdatePassiveState {
                target_id,
                passive_id: _,
                message: _,
            } => Some(*target_id),
            Effect::UpdateSkillState {
                target_id,
                skill_id: _,
                msg: _,
            } => Some(target_id.into()),
            _ => None,
        }
    }
}
