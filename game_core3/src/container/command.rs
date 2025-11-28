use crate::{skill::StaticSkillId, static_char::StaticCharId};

pub enum GameCoreActorCommand {
    TurnEnd,
    UseSkill {
        user: StaticCharId,
        skill: StaticSkillId,
    },
    GameStart,
}
