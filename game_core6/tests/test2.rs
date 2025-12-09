use std::sync::Arc;

use game_core6::{
    enemy::StaticEnemyDataInstance,
    game_core_actor::{GameCoreActor, GameCoreActorCommand},
    skill::SkillInstance,
    state::{CharData, EnemyData},
};
use test_utils::{char::char_1, enemy::SimpleEnemy1, skills};

#[test]
fn test_enemy_turn() {
    let mut core = GameCoreActor::new(
        vec![CharData {
            level: 1,
            data: char_1(),
            skills: vec![SkillInstance::new(skills::MagicAttuckSkill1)],
        }],
        Arc::new(vec![vec![EnemyData {
            data: StaticEnemyDataInstance::new(SimpleEnemy1),
            level: 1,
        }]]),
    )
    .unwrap();

    core.send_cmd(game_core6::game_core_actor::GameCoreActorCommand::GameStart);
    while core.forward().is_some() {}

    core.send_cmd(GameCoreActorCommand::TurnEnd);
    while core.forward().is_some() {}
}
