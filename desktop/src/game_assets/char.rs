use game_core6::{
    buttle_char::StaticCharData, passive::PassiveInstance, potential::Potential,
    skill::SkillInstance, state::CharData,
};

use crate::game_assets::skills::{Fireball, HonoonoOotatumaki, Kousituka};

fn empty_passives() -> Vec<PassiveInstance> {
    vec![]
}

pub(super) fn elena() -> CharData {
    CharData {
        data: StaticCharData {
            id: 1,
            name: "エレナ",
            potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
            passives: empty_passives,
        },
        level: 1,
        skills: vec![
            SkillInstance::new(Fireball),
            SkillInstance::new(Kousituka),
            SkillInstance::new(HonoonoOotatumaki),
        ],
    }
}

pub(super) fn yuko() -> CharData {
    CharData {
        data: StaticCharData {
            id: 2,
            name: "幽狐",
            potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
            passives: empty_passives,
        },
        level: 1,
        skills: vec![
            SkillInstance::new(Fireball),
            SkillInstance::new(Kousituka),
            SkillInstance::new(HonoonoOotatumaki),
        ],
    }
}

pub(super) fn yura() -> CharData {
    CharData {
        data: StaticCharData {
            id: 3,
            name: "ゆら",
            potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
            passives: empty_passives,
        },
        level: 1,
        skills: vec![
            SkillInstance::new(Fireball),
            SkillInstance::new(Kousituka),
            SkillInstance::new(HonoonoOotatumaki),
        ],
    }
}

pub(super) fn asya() -> CharData {
    CharData {
        data: StaticCharData {
            id: 4,
            name: "アーシャ",
            potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
            passives: empty_passives,
        },
        level: 1,
        skills: vec![
            SkillInstance::new(Fireball),
            SkillInstance::new(Kousituka),
            SkillInstance::new(HonoonoOotatumaki),
        ],
    }
}
