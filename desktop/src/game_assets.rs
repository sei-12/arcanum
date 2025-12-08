use std::sync::Arc;

use game_core6::{
    StaticCharId, StaticEnemyId,
    buttle_char::StaticCharData,
    damage::Damage,
    effect::Effect,
    enemy::{StaticEnemyData, StaticEnemyDataInstance},
    game_core_actor::GameCoreActor,
    passive::PassiveInstance,
    potential::Potential,
    skill::{SkillInstance, SkillTrait},
    state::{CharData, EnemyData},
};

pub fn new_game_core() -> GameCoreActor {
    GameCoreActor::new(
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
    .unwrap()
}

pub fn get_char_name(_static_char_id: StaticCharId) -> &'static str {
    "todo"
}

pub fn get_enemy_name(_static_enemy_id: StaticEnemyId) -> &'static str {
    "todo"
}

#[derive(Debug)]
struct Skill;

impl SkillTrait for Skill {
    fn call(
        &self,
        user_id: game_core6::runtime_id::RuntimeCharId,
        target_id: Option<game_core6::runtime_id::RuntimeEnemyId>,
        effector: &mut dyn game_core6::effector::EffectorTrait,
    ) -> Result<(), game_core6::WinOrLoseOrNextwave> {
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
