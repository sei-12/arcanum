use game_core9::{
    any_message::AnyMessageBox,
    buttle_char::ButtleCharArgs,
    buttle_enemy::{ButtleEnemyArgs, EnemyInfomation},
    core_actor::{GameCoreActor, UserInput},
    damage::DamageType,
    enemy_skill::{EnemySkill, EnemySkillAction, EnemySkillTarget},
    game_state::GameStateArgs,
    potential::Potential,
    skill::{SkillBox, SkillTrait},
    weapon::Weapon,
};

fn enemy_skill1() -> EnemySkill {
    EnemySkill {
        id: 1,
        name: "()",
        need_mp: 10.0,
        start_up_frames: 400,
        recovery_frame: 100,
        actions: vec![(
            EnemySkillTarget::Single,
            EnemySkillAction::Damage {
                ty: DamageType::Physics,
                dmg_mag: 1.0,
                count: 1,
            },
        )],
    }
}

fn enemy_skill2() -> EnemySkill {
    EnemySkill {
        id: 2,
        name: "()",
        need_mp: 10.0,
        start_up_frames: 10,
        recovery_frame: 10,
        actions: vec![],
    }
}

fn enemy_skill3() -> EnemySkill {
    EnemySkill {
        id: 3,
        name: "()",
        need_mp: 10.0,
        start_up_frames: 10,
        recovery_frame: 10,
        actions: vec![],
    }
}

#[derive(Debug, Clone)]
struct Skill {}
impl SkillTrait for Skill {
    fn info(&self) -> &game_core9::skill::SkillInfomation {
        &game_core9::skill::SkillInfomation {
            name: "ファイヤーボール",
            description: "",
            flaver_text: "",
            id: 1,
            default_need_mp: 10.0,
            defalut_hate: 10.0,
            defalut_cooldown: 10.0,
        }
    }
    fn tick(
        &self,
        _owner_id: game_core9::runtime_id::RuntimeCharId,
        _state: &game_core9::game_state::GameState,
        _effects_buffer: &mut std::collections::VecDeque<game_core9::effect::Effect>,
    ) {
    }
    fn start(&mut self) {}
    fn update(&mut self, _msg: &AnyMessageBox) {}
    fn current_progress(&self) -> Option<game_core9::skill::CharSkillProgress> {
        None
    }
}

fn elena() -> ButtleCharArgs {
    ButtleCharArgs {
        level: 1,
        potential: Potential::new(13.0, 8.0, 15.0, 6.0, 8.0),
        skills: vec![SkillBox::new(Skill {})],
        static_id: 1,
        name: "エレナ",
        weapon: Weapon {
            m_atk: 1.0,
            p_atk: 1.0,
            ty: game_core9::weapon::WeaponType::Cane,
        },
    }
}

fn yuuko() -> ButtleCharArgs {
    ButtleCharArgs {
        level: 1,
        potential: Potential::new(12.0, 10.0, 8.0, 10.0, 10.0),
        skills: vec![SkillBox::new(Skill {})],
        static_id: 3,
        name: "幽狐",
        weapon: Weapon {
            m_atk: 1.0,
            p_atk: 1.0,
            ty: game_core9::weapon::WeaponType::Cane,
        },
    }
}

fn asya() -> ButtleCharArgs {
    ButtleCharArgs {
        level: 1,
        potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
        skills: vec![SkillBox::new(Skill {})],
        static_id: 2,
        name: "アーシャ",
        weapon: Weapon {
            m_atk: 1.0,
            p_atk: 1.0,
            ty: game_core9::weapon::WeaponType::Cane,
        },
    }
}

fn nowaru() -> ButtleCharArgs {
    ButtleCharArgs {
        level: 1,
        potential: Potential::new(8.0, 13.0, 12.0, 6.0, 11.0),
        skills: vec![SkillBox::new(Skill {})],
        static_id: 4,
        name: "ノワール",
        weapon: Weapon {
            m_atk: 1.0,
            p_atk: 1.0,
            ty: game_core9::weapon::WeaponType::Cane,
        },
    }
}

fn args() -> GameStateArgs {
    GameStateArgs {
        chars: vec![elena(), yuuko(), asya(), nowaru()],
        enemy: ButtleEnemyArgs {
            action_patterns: vec![vec![1, 2, 3]],
            default_passive: vec![],
            info: EnemyInfomation {
                id: 1,
                desctiption: "",
                name: "Enemy1",
            },
            level: 1,
            potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
            skills: vec![enemy_skill1(), enemy_skill2(), enemy_skill3()],
        },
    }
}

pub fn game_core() -> GameCoreActor {
    GameCoreActor::new(args()).unwrap()
}
