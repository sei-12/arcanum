use game_core9::{
    damage::Damage,
    effect::Effect,
    skill::{CharSkillProgressKind, SkillTrait},
    skill_impl_utils::{SkillEffectUnit, SkillEffectUnits},
};

#[derive(Debug, Clone)]
pub struct Fireball {
    units: SkillEffectUnits,
}

impl Fireball {
    pub fn new() -> Self {
        let units = [
            SkillEffectUnit::new(3000, CharSkillProgressKind::Chanting, |_, _, _| {}).unwrap(),
            SkillEffectUnit::new(
                500,
                CharSkillProgressKind::Acting,
                |owner_id, state, effects_buffer| {
                    let enemy = state.get_enemy();
                    let owner = state.get_char(owner_id);
                    let dmg_mag = 1.1;
                    let dmg =
                        Damage::new_magic_damage(state, owner.lt_id(), enemy.lt_id(), dmg_mag);
                    effects_buffer.push_back(Effect::Damage(dmg));
                },
            )
            .unwrap(),
            SkillEffectUnit::new(500, CharSkillProgressKind::Acting, |_, _, _| {}).unwrap(),
        ];

        Fireball {
            units: SkillEffectUnits::new(units.to_vec()).unwrap(),
        }
    }
}

impl SkillTrait for Fireball {
    fn current_progress(&self) -> Option<game_core9::skill::CharSkillProgress> {
        self.units.current_progress()
    }

    fn info(&self) -> &game_core9::skill::SkillInfomation {
        &game_core9::skill::SkillInfomation {
            name: "ファイヤーボール",
            description: "敵に倍率1.1の魔法ダメージを与える",
            flaver_text: "",
            id: 2,
            default_need_mp: 50.0,
            defalut_hate: 50.0,
            defalut_cooldown: 1000.0,
        }
    }

    fn start(&mut self) {
        self.units.start().unwrap()
    }

    fn tick(
        &self,
        owner_id: game_core9::runtime_id::RuntimeCharId,
        state: &game_core9::game_state::GameState,
        effects_buffer: &mut std::collections::VecDeque<game_core9::effect::Effect>,
    ) {
        let msg = self.units.tick(owner_id, state, effects_buffer).unwrap();
        todo!()
        // effects_buffer.push_back(Effect::UpdateSkillState { skill_id: , msg });
    }

    fn update(&mut self, msg: &game_core9::any_message::AnyMessageBox) {}
}
