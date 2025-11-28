use crate::{WaveNum, args::EnemyData, buttle_enemy::ButtleEnemy};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeEnemyId {
    wave: usize,
    idx: usize,
}

pub(crate) trait ButtleEnemysItem {
    fn new(data: &EnemyData, id: RuntimeEnemyId) -> Self;
    fn is_dead(&self) -> bool;
}

pub struct ButtleEnemys {
    current_wave_idx: usize,
    inner: Vec<Vec<ButtleEnemy>>,
}


impl ButtleEnemys {
    pub(crate) fn new(enemy_data: &[Vec<EnemyData>]) -> Self {
        let waves = enemy_data
            .iter()
            .enumerate()
            .map(|(w, wave)| {
                wave.iter()
                    .enumerate()
                    .map(|(i, enemy)| ButtleEnemy::new(enemy, RuntimeEnemyId { wave: w, idx: i }))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert!(!waves.is_empty());
        debug_assert!({ waves.iter().all(|w| !w.is_empty()) });

        Self {
            current_wave_idx: 0,
            inner: waves,
        }
    }

    pub fn current_wave(&self) -> WaveNum {
        (self.current_wave_idx + 1) as WaveNum
    }

    pub fn get(&self, id: RuntimeEnemyId) -> &ButtleEnemy {
        &self.inner[id.wave][id.idx]
    }

    pub(crate) fn go_next_wave(&mut self) {
        assert!(!self.current_wave_is_last_wave());
        self.current_wave_idx += 1;
        assert!(self.current_wave_idx < self.inner.len());
    }

    pub(crate) fn current_wave_all_dead(&self) -> bool {
        self.inner[self.current_wave_idx]
            .iter()
            .all(|enemy| enemy.lt().is_dead())
    }

    pub(crate) fn current_wave_is_last_wave(&self) -> bool {
        self.current_wave_idx + 1 == self.inner.len()
    }

    pub fn current_wave_enemys(&self) -> &[ButtleEnemy] {
        &self.inner[self.current_wave_idx]
    }
}
