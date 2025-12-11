use std::sync::Arc;

use game_core6::{
    damage::DamageType,
    effect::Effect,
    enemy::StaticEnemyDataInstance,
    game_core_actor::{GameCoreActor, GameCoreActorCommand},
    output::{EffectedBy, Event, GameCoreOutput},
    skill::SkillInstance,
    state::{CharData, EnemyData},
};
use test_utils::{
    char::char_1,
    enemy::SimpleEnemy1,
    skills::{self, CustomSkillBuilder},
};

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

#[test]
fn test_game_end() {
    let skill = CustomSkillBuilder::new()
        .mag(100.0)
        .ty(DamageType::Magic)
        .build();

    let mut core = GameCoreActor::new(
        vec![CharData {
            level: 1,
            data: char_1(),
            skills: vec![skill],
        }],
        Arc::new(vec![vec![EnemyData {
            data: StaticEnemyDataInstance::new(SimpleEnemy1),
            level: 1,
        }]]),
    )
    .unwrap();

    core.send_cmd(GameCoreActorCommand::GameStart);
    let char = core.state().get_chars().first().unwrap();
    let char_id = char.runtime_id();
    let skill_id = char.skills().first().unwrap().runtime_id();
    while core.forward().is_some() {}

    core.send_cmd(GameCoreActorCommand::UseSkill {
        user_id: char_id,
        skill_id,
        target_id: None,
    });

    let output = core.forward().unwrap();
    assert!(
        matches!(output, GameCoreOutput::Event(Event::CharUseSkill)),
        "{:?}",
        output
    );

    let output = core.forward().unwrap();
    assert!(matches!(
        output,
        GameCoreOutput::Effect(EffectedBy::CharSkill(_), Effect::Damage(_))
    ));

    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Event(Event::Win)
    ));

    assert!(core.forward().is_none());
}

#[test]
fn test_go_next_wave() {
    let skill = CustomSkillBuilder::new()
        .mag(100.0)
        .ty(DamageType::Magic)
        .build();

    let mut core = GameCoreActor::new(
        vec![CharData {
            level: 1,
            data: char_1(),
            skills: vec![skill],
        }],
        Arc::new(vec![
            vec![EnemyData {
                data: StaticEnemyDataInstance::new(SimpleEnemy1),
                level: 1,
            }],
            vec![EnemyData {
                data: StaticEnemyDataInstance::new(SimpleEnemy1),
                level: 1,
            }],
        ]),
    )
    .unwrap();

    core.send_cmd(GameCoreActorCommand::GameStart);
    let char = core.state().get_chars().first().unwrap();
    let char_id = char.runtime_id();
    let skill_id = char.skills().first().unwrap().runtime_id();
    while core.forward().is_some() {}

    core.send_cmd(GameCoreActorCommand::UseSkill {
        user_id: char_id,
        skill_id,
        target_id: None,
    });

    let output = core.forward().unwrap();
    assert!(
        matches!(output, GameCoreOutput::Event(Event::CharUseSkill)),
        "{:?}",
        output
    );

    let output = core.forward().unwrap();
    assert!(matches!(
        output,
        GameCoreOutput::Effect(EffectedBy::CharSkill(_), Effect::Damage(_))
    ));

    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Event(Event::GoNextWave)
    ));
    

    while let Some(output) = core.forward() {
        dbg!(output);
    }

}
