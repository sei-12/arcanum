use rand::{Rng, rngs::ThreadRng};

use crate::{container::Container, lt::Potential};

pub type StaticEnemyId = usize;

pub type ActionFunc = fn(con: &mut Container);

mod enemy0;

/// 一度に表示されている行動の数
/// 7の場合7ターン先の行動まで見えるという意味
pub const NUM_VIEW_ENEMY_ACTION: usize = 7;

#[derive(Debug, Clone, Copy)]
pub enum EnemyAction {
    Low,
    Mid,
    High,
    Assist,
    Interference,
}

impl EnemyAction {
    pub(crate) fn random(rng: &mut ThreadRng) -> Self {
        const ACTIONS: [EnemyAction; 5] = [
            EnemyAction::Low,
            EnemyAction::Assist,
            EnemyAction::High,
            EnemyAction::Interference,
            EnemyAction::Mid,
        ];
        ACTIONS[rng.random_range(0..ACTIONS.len())]
    }
}

#[derive(Debug)]
pub struct StaticEnemyData {
    pub name: &'static str,
    pub id: StaticEnemyId,
    pub potential: Potential,
    low: ActionFunc,
    mid: ActionFunc,
    high: ActionFunc,
    assist: ActionFunc,
    interference: ActionFunc,
}

impl StaticEnemyData {
    pub(crate) fn play_action(&self, action: EnemyAction, con: &mut Container) {
        match action {
            EnemyAction::Assist => (self.assist)(con),
            EnemyAction::High => (self.high)(con),
            EnemyAction::Interference => (self.interference)(con),
            EnemyAction::Low => (self.low)(con),
            EnemyAction::Mid => (self.mid)(con),
        }
    }
    pub fn get(id: StaticEnemyId) -> Option<&'static StaticEnemyData> {
        const ENEMYS: [&StaticEnemyData; 1] = [&enemy0::ENEMY];
        ENEMYS.get(id).copied()
    }
}
// mod log_tmplate {
//     use crate::enemy_ai::StaticEnemyData;

//     pub fn ltmp_assist(enemy: &StaticEnemyData) -> String {
//         format!("{}の補助行動", enemy.name)
//     }
// }
