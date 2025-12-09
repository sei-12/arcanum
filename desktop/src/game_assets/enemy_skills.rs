use game_core6::enemy::{EnemySkillInsance, StaticEnemySkillData};

pub(super) struct EnemySkill1;
impl StaticEnemySkillData for EnemySkill1 {
    fn call(
        &self,
        user_id: game_core6::runtime_id::RuntimeEnemyId,
        effector: &mut dyn game_core6::effector::EffectorTrait,
    ) -> Result<(), game_core6::WinOrLoseOrNextwave> {
        Ok(())
    }

    fn clone(&self) -> game_core6::enemy::EnemySkillInsance {
        EnemySkillInsance::new(Self)
    }

    fn static_id(&self) -> game_core6::StaticEnemySkillId {
        1
    }
    fn description(&self) -> &'static str {
        "todo"
    }
    fn name(&self) -> &'static str {
        "todo"
    }
}



pub(super) struct EnemySkill2;
impl StaticEnemySkillData for EnemySkill2 {
    fn call(
        &self,
        user_id: game_core6::runtime_id::RuntimeEnemyId,
        effector: &mut dyn game_core6::effector::EffectorTrait,
    ) -> Result<(), game_core6::WinOrLoseOrNextwave> {
        Ok(())
    }

    fn clone(&self) -> game_core6::enemy::EnemySkillInsance {
        EnemySkillInsance::new(Self)
    }

    fn static_id(&self) -> game_core6::StaticEnemySkillId {
        1
    }
    fn description(&self) -> &'static str {
        "todo"
    }
    fn name(&self) -> &'static str {
        "todo"
    }
}


