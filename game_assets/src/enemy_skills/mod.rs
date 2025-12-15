use game_core7::{
    damage::DamageType,
    enemy_skill::{EnemySkill, EnemySkillAction, EnemySkillTarget},
    passive::passive_box::PassiveBox,
};

use crate::passives;

pub fn hikkaku() -> EnemySkill {
    EnemySkill {
        id: 1,
        name: "引っ掻く",
        need_mp: 50.0,
        start_up_frames: 300,
        recovery_frame: 200,
        actions: vec![(
            EnemySkillTarget::Single,
            EnemySkillAction::Damage {
                ty: DamageType::Physics,
                dmg_mag: 1.0,
                count: 2,
            },
        )],
    }
}

pub fn otakebi() -> EnemySkill {
    EnemySkill {
        id: 2,
        name: "ウォークライ",
        need_mp: 30.0,
        start_up_frames: 300,
        recovery_frame: 300,
        actions: vec![(
            EnemySkillTarget::Self_,
            EnemySkillAction::AddPassive(PassiveBox::new(passives::Warcry { time: 1000 })),
        )],
    }
}

pub fn iwanage() -> EnemySkill {
    EnemySkill {
        id: 3,
        name: "岩投げ",
        need_mp: 100.0,
        start_up_frames: 350,
        recovery_frame: 250,
        actions: vec![(
            EnemySkillTarget::Single,
            EnemySkillAction::Damage {
                ty: DamageType::Physics,
                dmg_mag: 1.5,
                count: 1,
            },
        )],
    }
}
