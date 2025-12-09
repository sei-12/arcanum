use std::sync::Arc;

use game_core6::{
    effect::Effect,
    enemy::StaticEnemyDataInstance,
    game_core_actor::{GameCoreActor, GameCoreActorCommand},
    output::{EffectedBy, Event, GameCoreOutput},
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
    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Event(Event::EnemyTurnStart)
    ));
    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Effect(
            EffectedBy::GameSystem,
            Effect::HealSp {
                target_id: _,
                num: _
            }
        )
    ));

    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Event(Event::EnemyUseSkill)
    ));

    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Effect(EffectedBy::EnemySkill(_), Effect::Damage(_))
    ));

    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Event(Event::PlayerTurnStart)
    ));

    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Effect(EffectedBy::GameSystem, Effect::HealMp { num: _ })
    ));

    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Effect(
            EffectedBy::GameSystem,
            Effect::HealSkillCooldownAll {
                target_id: _,
                num: _
            }
        )
    ));

    let output = core.forward().unwrap();
    assert!(matches!(output, GameCoreOutput::WaitInput), "{:?}", output);
}
