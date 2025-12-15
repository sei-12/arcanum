use game_core7::{LevelNum, buttle_enemy::ButtleEnemyArgs, enemy_skill, potential::Potential};

use crate::enemy_skills;

pub fn enemy1(level: LevelNum) -> ButtleEnemyArgs {
    ButtleEnemyArgs {
        level,
        info: game_core7::buttle_enemy::EnemyInfomation {
            name: "ゴブリン",
            desctiption: "",
        },
        potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
        skills: vec![
            enemy_skills::hikkaku(),
            enemy_skills::iwanage(),
            enemy_skills::otakebi(),
        ],
        action_patterns: vec![
            vec![0, 0, 1],
            vec![1, 0, 1],
            vec![0, 1, 2],
            vec![1, 1, 2, 2],
        ],
        default_passive: vec![],
    }
}
