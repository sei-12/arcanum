use criterion::{Criterion, criterion_group, criterion_main};

mod a {

    use game_core9::{
        any_message::AnyMessageBox,
        buttle_char::ButtleCharArgs,
        buttle_enemy::{ButtleEnemyArgs, EnemyInfomation},
        core_actor::{EffectsBuffer, GameCoreActor, UserInput},
        enemy_skill::EnemySkill,
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
            start_up_frames: 10,
            recovery_frame: 10,
            actions: vec![],
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
                name: "",
                description: "",
                flaver_text: "",
                id: 1,
                default_need_mp: 10.0,
                defalut_hate: 10.0,
                defalut_cooldown: 10.0,
            }
        }
        fn current_progress(&self) -> Option<game_core9::skill::CharSkillProgress> {
            None
        }
        fn tick(
            &self,
            _owner_id: game_core9::runtime_id::RuntimeSkillId,
            _state: &game_core9::game_state::GameState,
            _effects_buffer: &mut EffectsBuffer,
        ) {
        }
        fn start(&mut self) {}
        fn update(&mut self, _msg: &AnyMessageBox) {}
    }

    fn char1() -> ButtleCharArgs {
        ButtleCharArgs {
            level: 1,
            name: "",
            potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
            skills: vec![SkillBox::new(Skill {})],
            static_id: 1,
            weapon: Weapon {
                m_atk: 1.0,
                p_atk: 1.0,
                ty: game_core9::weapon::WeaponType::Cane,
            },
        }
    }
    fn args() -> GameStateArgs {
        GameStateArgs {
            chars: vec![char1()],
            enemy: ButtleEnemyArgs {
                action_patterns: vec![vec![1, 2, 3]],
                default_passive: vec![],
                info: EnemyInfomation {
                    desctiption: "",
                    name: "",
                    id: 1,
                },
                level: 1,
                potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
                skills: vec![enemy_skill1(), enemy_skill2(), enemy_skill3()],
            },
        }
    }

    pub fn main() {
        let mut output_buffer = Vec::new();
        let mut core = GameCoreActor::new(args()).unwrap();
        for _ in 0..100 {
            let res = core.tick(UserInput::None, &mut output_buffer);
            assert!(res.is_ok())
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("normal", |b| {
        b.iter(|| {
            a::main();
        })
    });
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
