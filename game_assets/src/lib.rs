use game_core7::{
    buttle_enemy::{ButtleEnemyArgs, EnemyInfomation},
    core_actor::CoreActor,
    enemy_skill::EnemySkill,
    potential::Potential,
    state::GameStateArgs,
};

use crate::enemys::enemy1;

pub mod chars;
pub mod enemy_skills;
pub mod enemys;
pub mod passives;
pub mod skill;

const FPS: u64 = 100;

fn args() -> GameStateArgs {
    GameStateArgs {
        chars: vec![chars::elena(), chars::asya(), chars::yuuko(), chars::kazu()],
        enemy: enemy1(1),
    }
}

fn a() {
    let core = CoreActor::new(args()).unwrap();
}
