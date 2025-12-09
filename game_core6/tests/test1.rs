use std::sync::Arc;

use game_core6::{
    TURN_START_HEAL_MP_NUM,
    buttle_char::StaticCharData,
    damage::{Damage, DamageType},
    effect::Effect,
    enemy::{StaticEnemyData, StaticEnemyDataInstance},
    game_core_actor::{GameCoreActor, GameCoreActorCommand},
    output::{EffectedBy, Event, GameCoreOutput},
    passive::PassiveInstance,
    potential::Potential,
    skill::{SkillCost, SkillInstance, SkillTrait},
    state::{CharData, EnemyData},
};

#[derive(Debug)]
struct Skill;

impl SkillTrait for Skill {
    fn call(
        &self,
        user_id: game_core6::runtime_id::RuntimeCharId,
        _skill_id: game_core6::runtime_id::RuntimeSkillId,
        target_id: Option<game_core6::runtime_id::RuntimeEnemyId>,
        effector: &mut dyn game_core6::effector::EffectorTrait,
    ) -> Result<SkillCost, game_core6::WinOrLoseOrNextwave> {
        let target = effector
            .state()
            .get_enemys_highest_target_priority(target_id)
            .next()
            .unwrap();

        effector.accept_effect(Effect::Damage(Damage::new_magic_damage(
            effector.state(),
            user_id.into(),
            target.lt_id(),
            1.0,
        )))?;

        Ok(SkillCost {
            mp: 10,
            hate: 10,
            cooldown: 10,
        })
    }

    fn need_mp(&self, _state: &game_core6::state::GameState) -> game_core6::MpNum {
        10
    }

    fn clone(&self) -> game_core6::skill::SkillInstance {
        SkillInstance::new(Self)
    }

    fn update(&mut self, _msg: &game_core6::skill::SkillUpdateMessage) {}

    fn doc(&self) -> &game_core6::skill::SkillDocument {
        &game_core6::skill::SkillDocument {
            name: "a",
            description: "description",
            id: 1,
            default_need_mp: 10,
            defalut_hate: 10,
            defalut_cooldown: 10,
        }
    }
}

fn passives() -> Vec<PassiveInstance> {
    vec![]
}

const ENEMY_POTENTIAL: Potential = Potential::new(10.0, 10.0, 10.0, 10.0, 10.0);
#[derive(Debug)]
struct Enemy;
impl StaticEnemyData for Enemy {
    fn clone(&self) -> game_core6::enemy::StaticEnemyDataInstance {
        StaticEnemyDataInstance::new(Self)
    }
    fn potential(&self) -> &Potential {
        &ENEMY_POTENTIAL
    }
    fn select_skill(
        &self,
        _user_id: game_core6::runtime_id::RuntimeEnemyId,
        _state: &game_core6::state::GameState,
    ) -> game_core6::enemy::EnemySkillInsance {
        todo!()
    }
    fn static_id(&self) -> game_core6::StaticEnemyId {
        1
    }

    fn name(&self) -> &'static str {
        "enemy1"
    }
}

#[test]
fn test_game_start() {
    let mut core = GameCoreActor::new(
        vec![CharData {
            level: 1,
            data: StaticCharData {
                id: 1,
                name: "char1",
                passives,
                potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
            },
            skills: vec![SkillInstance::new(Skill)],
        }],
        Arc::new(vec![vec![EnemyData {
            level: 1,
            data: StaticEnemyDataInstance::new(Enemy),
        }]]),
    )
    .unwrap();

    assert!(core.forward().is_none());

    core.send_cmd(GameCoreActorCommand::GameStart);

    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Event(Event::PlayerTurnStart)
    ));

    assert_eq!(core.state().player_mp(), 0);

    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Effect(EffectedBy::GameSystem, Effect::HealMp { num: _ })
    ));

    assert_eq!(core.state().player_mp(), TURN_START_HEAL_MP_NUM);

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

    assert!(matches!(core.forward().unwrap(), GameCoreOutput::WaitInput));
}

#[test]
fn test_game() {
    let mut core = GameCoreActor::new(
        vec![CharData {
            level: 1,
            data: StaticCharData {
                id: 1,
                name: "char1",
                passives,
                potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
            },
            skills: vec![SkillInstance::new(Skill)],
        }],
        Arc::new(vec![vec![EnemyData {
            level: 1,
            data: StaticEnemyDataInstance::new(Enemy),
        }]]),
    )
    .unwrap();

    assert!(core.forward().is_none());

    core.send_cmd(GameCoreActorCommand::GameStart);

    let char_id = core.state().get_chars().first().unwrap().runtime_id();
    let enemy_id = core
        .state()
        .get_current_wave_enemys()
        .first()
        .unwrap()
        .runtime_id();

    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Event(Event::PlayerTurnStart)
    ));

    assert_eq!(core.state().player_mp(), 0);

    assert!(matches!(
        core.forward().unwrap(),
        GameCoreOutput::Effect(EffectedBy::GameSystem, Effect::HealMp { num: _ })
    ));

    assert_eq!(core.state().player_mp(), TURN_START_HEAL_MP_NUM);

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

    assert!(matches!(core.forward().unwrap(), GameCoreOutput::WaitInput));

    let char = core.state().get_chars().first().unwrap();
    let skill = char.skills().first().unwrap();

    core.send_cmd(GameCoreActorCommand::UseSkill {
        user_id: char.runtime_id(),
        target_id: None,
        skill_id: skill.runtime_id(),
    });

    let output = core.forward().unwrap();
    assert!(
        matches!(output, GameCoreOutput::Event(Event::CharUseSkill)),
        "{:?}",
        output
    );

    let output = core.forward().unwrap();
    let GameCoreOutput::Effect(_, Effect::Damage(dmg)) = output else {
        panic!();
    };
    assert_eq!(dmg.causer().to_lt_id().unwrap(), char_id.into());
    assert_eq!(dmg.target(), enemy_id.into());
    assert_eq!(dmg.ty(), DamageType::Magic);
    assert_eq!(
        dmg.dmg(),
        core.state().get_char(char_id).lt().magic_attuck()
    );

    // 敵の残りHPの確認
    let enemy = core.state().get_current_wave_enemy(enemy_id).lt();
    assert_eq!(
        enemy.hp().round(),
        (enemy.max_hp() - dmg.dmg()).round(),
        "敵の残りHPが一致しない"
    );

    let output = core.forward().unwrap();
    assert!(
        matches!(
            output,
            GameCoreOutput::Effect(EffectedBy::SkillCost, Effect::ConsumeMp { num: _ })
        ),
        "{:?}",
        output
    );

    let output = core.forward().unwrap();
    assert!(
        matches!(
            output,
            GameCoreOutput::Effect(
                EffectedBy::SkillCost,
                Effect::AddHate {
                    target_id: _,
                    num: _
                }
            )
        ),
        "{:?}",
        output
    );

    let output = core.forward().unwrap();
    assert!(
        matches!(
            output,
            GameCoreOutput::Effect(
                EffectedBy::SkillCost,
                Effect::SetSkillCooldown {
                    target_id: _,
                    skill_id: _,
                    num: _
                }
            )
        ),
        "{:?}",
        output
    );

    let output = core.forward().unwrap();
    assert!(matches!(output, GameCoreOutput::WaitInput), "{:?}", output);
}
