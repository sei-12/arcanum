use crate::{
    buttle_enemy::{
        StaticEnemyData,
        skill::{hikkaku, isinage},
        static_datas::{EnemyAbility, EnemyActionText},
    },
    enemys::RuntimeEnemyId,
    event_accepter::{EventAccepter, WinOrLoseOrNextwave},
    potential::Potential,
    state::GameState,
};

// #[derive(Debug, Clone)]
// pub struct Goblin;

const POTENTIAL: Potential = Potential::new(3.0, 13.0, 13.0, 7.0, 14.0);

pub(super) const DATA: StaticEnemyData = StaticEnemyData {
    name: "ゴブリン",
    potential: &POTENTIAL,
    abilitys: &[EnemyAbility::Revenge],
    action_text: &[
        EnemyActionText::IfThenReturn {
            condition: "SPが80以上ある",
            skill_doc: &hikkaku::DOCUMENT,
        },
        EnemyActionText::Normal(&isinage::DOCUMENT),
    ],

    action_fn: action,
};

fn action(
    enemy_id: RuntimeEnemyId,
    accepter: &mut EventAccepter,
    state: &mut GameState,
) -> Result<(), WinOrLoseOrNextwave> {
    if state.enemys().get(enemy_id).sp() >= 80 {
        hikkaku::call(enemy_id, state, accepter)?;
    }else{
        isinage::call(enemy_id, state, accepter)?;
    }
    Ok(())
}

// impl StaticEnemyTrait for Goblin {
//     fn name(&self) -> &'static str {
//         "ゴブリン"
//     }

//     fn potential(&self) -> &'static crate::potential::Potential {
//         &POTENTIAL
//     }

//     fn action(
//         &self,
//         enemy_id: crate::enemys::RuntimeEnemyId,
//         state: &crate::state::GameState,
//         events: &mut impl crate::event::EventsQuePusher,
//     ) {
//         if state.enemys().get(enemy_id).sp() >= 80 {
//             hikkaku::call(enemy_id, state, events);
//         } else {
//             isinage::call(enemy_id, state, events);
//         }
//     }

//     fn abilitys(&self) -> &'static [EnemyAbility] {
//         &[EnemyAbility::Revenge]
//     }

//     fn action_text(&self) -> &'static [EnemyActionText] {
//         &[
//             EnemyActionText::IfThenReturn {
//                 condition: "SPが80以上ある",
//                 skill_doc: &hikkaku::DOCUMENT,
//             },
//             EnemyActionText::Normal(&isinage::DOCUMENT),
//         ]
//     }
// }
