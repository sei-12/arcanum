use crate::{
    buttle_enemy::{
        skill::{hikkaku, isinage},
        static_datas::{EnemyAbility, EnemyActionText, StaticEnemyTrait},
    },
    potential::Potential,
};

#[derive(Debug)]
pub struct Goblin;

const POTENTIAL: Potential = Potential::new(3.0, 13.0, 13.0, 7.0, 14.0);

impl StaticEnemyTrait for Goblin {
    fn name(&self) -> &'static str {
        "ゴブリン"
    }

    fn potential(&self) -> &'static crate::potential::Potential {
        &POTENTIAL
    }

    fn action(
        &self,
        enemy_id: crate::enemys::RuntimeEnemyId,
        state: &crate::state::GameState,
        events: &mut impl crate::event::EventsQuePusher,
    ) {
        if state.enemys().get(enemy_id).sp() >= 80 {
            hikkaku::call(enemy_id, state, events);
        } else {
            isinage::call(enemy_id, state, events);
        }
    }

    fn abilitys(&self) -> &'static [EnemyAbility] {
        &[EnemyAbility::Revenge]
    }

    fn action_text(&self) -> &'static [EnemyActionText] {
        &[
            EnemyActionText::IfThenReturn {
                condition: "SPが80以上ある",
                skill_name: hikkaku::SKILL_NAME,
            },
            EnemyActionText::Normal(isinage::SKILL_NAME),
        ]
    }
}
