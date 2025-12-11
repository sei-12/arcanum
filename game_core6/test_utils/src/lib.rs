use game_core6::potential::Potential;

pub const SIMPLE_POTENTIAL: Potential = Potential::new(10.0, 10.0, 10.0, 10.0, 10.0);

pub mod skills {
    use game_core6::{
        CooldownNum, HateNum, MpNum, StatusNum,
        damage::{Damage, DamageType},
        skill::{SkillCost, SkillInfomation, SkillInstance, SkillTrait},
    };

    #[derive(Debug)]
    /// 倍率1.0の単体魔法攻撃
    pub struct MagicAttuckSkill1;
    impl SkillTrait for MagicAttuckSkill1 {
        fn info(&self) -> &game_core6::skill::SkillInfomation {
            &game_core6::skill::SkillInfomation {
                name: "skill1",
                description: "skill1",
                id: 10001,
                default_need_mp: 100,
                defalut_hate: 100,
                defalut_cooldown: 100,
            }
        }

        fn call(
            &self,
            user_id: game_core6::runtime_id::RuntimeCharId,
            _skill_id: game_core6::runtime_id::RuntimeSkillId,
            target_id: Option<game_core6::runtime_id::RuntimeEnemyId>,
            effector: &mut dyn game_core6::effector::EffectorTrait,
        ) -> Result<game_core6::skill::SkillCost, game_core6::WinOrLoseOrNextwave> {
            let target = effector
                .state()
                .get_enemys_highest_target_priority(target_id)
                .next()
                .unwrap();

            let dmg =
                Damage::new_magic_damage(effector.state(), user_id.into(), target.lt_id(), 1.0);

            effector.accept_effect(game_core6::effect::Effect::Damage(dmg))?;

            Ok(SkillCost::from_defalut(self.info()))
        }

        fn clone_instance(&self) -> SkillInstance {
            SkillInstance::new(Self)
        }
    }

    #[derive(Debug)]
    /// 倍率1.0の単体物理攻撃
    pub struct PhysicsAttuckSkill1;
    impl SkillTrait for PhysicsAttuckSkill1 {
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

            let dmg =
                Damage::new_physics_damage(effector.state(), user_id.into(), target.lt_id(), 1.0);

            effector.accept_effect(game_core6::effect::Effect::Damage(dmg))?;

            Ok(SkillCost::from_defalut(self.info()))
        }

        fn info(&self) -> &game_core6::skill::SkillInfomation {
            &game_core6::skill::SkillInfomation {
                name: "skill2",
                description: "skill2",
                id: 10002,
                default_need_mp: 100,
                defalut_hate: 100,
                defalut_cooldown: 100,
            }
        }

        fn clone_instance(&self) -> SkillInstance {
            SkillInstance::new(Self)
        }
    }

    /// 単体攻撃スキル
    pub struct CustomSkillBuilder {
        mag: StatusNum,
        ty: Option<DamageType>,
        need_mp: MpNum,
        hate: HateNum,
        cooldown: CooldownNum,
    }

    impl CustomSkillBuilder {
        pub fn new() -> Self {
            Self {
                mag: 1.0,
                ty: None,
                need_mp: 0,
                hate: 0,
                cooldown: 0,
            }
        }

        pub fn mag(mut self, value: StatusNum) -> Self {
            self.mag = value;
            self
        }

        pub fn ty(mut self, value: DamageType) -> Self {
            self.ty = Some(value);
            self
        }

        pub fn need_mp(mut self, value: MpNum) -> Self {
            self.need_mp = value;
            self
        }

        pub fn hate(mut self, value: HateNum) -> Self {
            self.hate = value;
            self
        }

        pub fn cooldown(mut self, value: CooldownNum) -> Self {
            self.cooldown = value;
            self
        }

        pub fn build(self) -> SkillInstance {
            SkillInstance::new(CustomAttuck {
                mag: self.mag,
                ty: self.ty.unwrap_or_else(|| panic!()),
                info: SkillInfomation {
                    name: "()",
                    description: "()",
                    id: 10003,
                    default_need_mp: self.need_mp,
                    defalut_hate: self.hate,
                    defalut_cooldown: self.cooldown,
                },
            })
        }
    }

    impl Default for CustomSkillBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    #[derive(Debug, Clone)]
    struct CustomAttuck {
        mag: StatusNum,
        ty: DamageType,
        info: SkillInfomation,
    }
    impl SkillTrait for CustomAttuck {
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

            let dmg = match self.ty {
                DamageType::Magic => Damage::new_magic_damage(
                    effector.state(),
                    user_id.into(),
                    target.lt_id(),
                    self.mag,
                ),
                DamageType::Physics => Damage::new_physics_damage(
                    effector.state(),
                    user_id.into(),
                    target.lt_id(),
                    self.mag,
                ),
                DamageType::Fixed => unimplemented!(),
            };

            effector.accept_effect(game_core6::effect::Effect::Damage(dmg))?;

            Ok(SkillCost::from_defalut(self.info()))
        }

        fn info(&self) -> &game_core6::skill::SkillInfomation {
            &self.info
        }

        fn clone_instance(&self) -> SkillInstance {
            SkillInstance::new(self.clone())
        }
    }
}

pub mod enemy {
    use game_core6::enemy::{EnemySkillInsance, StaticEnemyData, StaticEnemyDataInstance};

    use crate::{SIMPLE_POTENTIAL, enemy_skill::PhysicsAttuckEnemySkill1};

    /// ポテンシャルは全て10
    /// スキルは倍率1.0の単体物理攻撃のみ
    #[derive(Debug)]
    pub struct SimpleEnemy1;
    impl StaticEnemyData for SimpleEnemy1 {
        fn clone(&self) -> game_core6::enemy::StaticEnemyDataInstance {
            StaticEnemyDataInstance::new(Self)
        }

        fn name(&self) -> &'static str {
            ""
        }

        fn potential(&self) -> &game_core6::potential::Potential {
            &SIMPLE_POTENTIAL
        }

        fn select_skill(
            &self,
            _user_id: game_core6::runtime_id::RuntimeEnemyId,
            _state: &game_core6::state::GameState,
        ) -> game_core6::enemy::EnemySkillInsance {
            EnemySkillInsance::new(PhysicsAttuckEnemySkill1)
        }

        fn static_id(&self) -> game_core6::StaticEnemyId {
            10001
        }
    }
}

pub mod enemy_skill {
    use game_core6::{
        damage::Damage,
        enemy::{EnemySkillInsance, StaticEnemySkillData},
    };

    #[derive(Debug)]
    pub struct PhysicsAttuckEnemySkill1;
    impl StaticEnemySkillData for PhysicsAttuckEnemySkill1 {
        fn call(
            &self,
            _user_id: game_core6::runtime_id::RuntimeEnemyId,
            effector: &mut dyn game_core6::effector::EffectorTrait,
        ) -> Result<(), game_core6::WinOrLoseOrNextwave> {
            let target = effector.state().get_highest_hate_char();
            let dmg =
                Damage::new_physics_damage(effector.state(), _user_id.into(), target.lt_id(), 1.0);
            effector.accept_effect(game_core6::effect::Effect::Damage(dmg))?;
            Ok(())
        }

        fn clone(&self) -> game_core6::enemy::EnemySkillInsance {
            EnemySkillInsance::new(Self)
        }

        fn description(&self) -> &'static str {
            ""
        }

        fn name(&self) -> &'static str {
            ""
        }
        fn static_id(&self) -> game_core6::StaticEnemySkillId {
            1
        }
    }
}

pub mod char {
    use game_core6::{buttle_char::StaticCharData, passive::PassiveInstance};

    use crate::SIMPLE_POTENTIAL;

    fn empty_passives() -> Vec<PassiveInstance> {
        Vec::new()
    }

    pub fn char_1() -> StaticCharData {
        StaticCharData {
            id: 10001,
            name: "hello",
            passives: empty_passives,
            potential: SIMPLE_POTENTIAL.clone(),
        }
    }
}
