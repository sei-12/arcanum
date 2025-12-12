use crate::{
    CooldownNum, HateNum, MpNum, StaticCharId, StaticPassiveId, StaticSkillId, StatusNum, any_message::AnyMessage, damage, runtime_id::LtId
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
        target_id: StaticCharId,
        skill_id: StaticSkillId,
        num: CooldownNum,
    },
    HealSkillCooldown {
        target_id: StaticCharId,
        skill_id: StaticSkillId,
        num: CooldownNum,
    },
    HealSkillCooldownAll {
        target_id: StaticCharId,
        num: CooldownNum,
    },
    HealHate {
        target_id: StaticCharId,
        num: HateNum,
    },
    AddHate {
        target_id: StaticCharId,
        num: HateNum,
    },
    UpdatePassiveState {
        target: LtId,
        passive_id: StaticPassiveId,
        message: AnyMessage,
    },
    Damage(damage::Damage),
    UseSkill {
        
    },
    // UpdatePassiveState {
    //     target_id: LtId,
    //     passive_id: StaticPassiveId,
    //     message: PassiveUpdateMessage ,
    // },
    // AddPassive {
    //     target_id: LtId,
    //     passive: PassiveInstance,
    // },
    // UpdateSkillState {
    //     target_id: StaticCharId,
    //     skill_id: StaticSkillId,
    //     msg: SkillUpdateMessage,
    // },
}

// impl Effect {
//     /// ターゲットを持っているタイプのエフェクトならターゲットを返す
//     pub(crate) fn target(&self) -> Option<LtId> {
//         match self {
//             Effect::AddHate { target_id, num: _ } => Some(target_id.into()),
//             Effect::AddPassive {
//                 target_id,
//                 passive: _,
//             } => Some(*target_id),
//             Effect::ConsumeSp { target_id, num: _ } => Some(target_id.into()),
//             Effect::Damage(dmg) => Some(dmg.target()),
//             Effect::HealHp { target_id, num: _ } => Some(*target_id),
//             Effect::HealSkillCooldown {
//                 target_id,
//                 skill_id: _,
//                 num: _,
//             } => Some(target_id.into()),
//             Effect::HealSkillCooldownAll { target_id, num: _ } => Some(target_id.into()),
//             Effect::HealSp { target_id, num: _ } => Some(target_id.into()),
//             Effect::SetSkillCooldown {
//                 target_id,
//                 skill_id: _,
//                 num: _,
//             } => Some(target_id.into()),
//             Effect::UpdatePassiveState {
//                 target_id,
//                 passive_id: _,
//                 message: _,
//             } => Some(*target_id),
//             Effect::UpdateSkillState {
//                 target_id,
//                 skill_id: _,
//                 msg: _,
//             } => Some(target_id.into()),
//             _ => None,
//         }
//     }
// }
