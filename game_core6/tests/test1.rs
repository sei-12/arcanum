use std::sync::Arc;

use game_core6::{
    TURN_START_HEAL_MP_NUM,
    buttle_char::StaticCharData,
    effect::Effect,
    enemy::{StaticEnemyData, StaticEnemyDataInstance},
    game_core_actor::{GameCoreActor, GameCoreActorCommand},
    output::{EffectedBy, Event, GameCoreOutput},
    passive::PassiveInstance,
    potential::Potential,
    skill::{SkillInstance, SkillTrait},
    state::{CharData, EnemyData},
};

#[derive(Debug)]
struct Skill;

impl SkillTrait for Skill {
    fn call(
        &self,
        _user: game_core6::runtime_id::RuntimeCharId,
        _effector: &mut dyn game_core6::effector::EffectorTrait,
    ) -> Result<(), game_core6::WinOrLoseOrNextwave> {
        Ok(())
    }
    fn clone(&self) -> game_core6::skill::SkillInstance {
        SkillInstance::new(Self)
    }
    fn static_id(&self) -> game_core6::StaticSkillId {
        1
    }
    fn update(&mut self, _msg: &game_core6::skill::SkillUpdateMessage) {}
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
}

#[test]
fn test_game_start() {
    let mut core = GameCoreActor::new(
        vec![CharData {
            level: 1,
            data: StaticCharData {
                id: 1,
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
