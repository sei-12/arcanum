use std::sync::Arc;

use game_core6::{
    StaticCharId, StaticEnemyId,
    buttle_char::StaticCharData,
    damage::Damage,
    effect::Effect,
    enemy::{EnemySkillInsance, StaticEnemyData, StaticEnemyDataInstance},
    game_core_actor::{GameCoreActor, GameCoreActorCommand},
    passive::PassiveInstance,
    potential::Potential,
    skill::{SkillInstance, SkillTrait},
    state::{CharData, EnemyData},
};

use crate::game_assets::{
    char::{asya, elena, yuko, yura},
    skills::{Fireball, Kousituka},
};

mod char;
mod enemy_skills;
mod passive;
mod skills;

pub fn new_game_core() -> GameCoreActor {
    let mut core = GameCoreActor::new(
        vec![elena(), yuko(), yura(), asya()],
        Arc::new(vec![vec![EnemyData {
            level: 1,
            data: StaticEnemyDataInstance::new(Enemy),
        }]]),
    )
    .unwrap();

    core.send_cmd(GameCoreActorCommand::GameStart);

    loop {
        if core.forward().is_none() {
            break;
        }
    }

    core
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
        EnemySkillInsance::new(enemy_skills::EnemySkill1)
    }
    fn static_id(&self) -> game_core6::StaticEnemyId {
        1
    }
    fn name(&self) -> &'static str {
        "name"
    }
}
