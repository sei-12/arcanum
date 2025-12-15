use crate::StaticSkillId;

pub struct GameCoreActor {}

impl GameCoreActor {
    pub fn new() -> Result<Self, crate::Error> {
        todo!()
    }

    pub fn tick(&mut self) {}
}

pub enum GameCoreOutput {
    Effect(OutputEffect),
}

pub enum EffectedBy {
    Skill { id: StaticSkillId, num: u32 },
}

pub struct OutputEffect {
    pub by: EffectedBy,
    pub kind: OutputEffectKind,
}

pub enum OutputEffectKind {
    Damage,
}
