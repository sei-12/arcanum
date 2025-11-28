use std::fmt::Debug;

use crate::{WaveNum, args::EnemyData};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeEnemyId {
    wave_idx: usize,
    idx: usize,
}

pub trait ButtleEnemysItem {
    fn new(data: &EnemyData, id: RuntimeEnemyId) -> Self;
    fn is_dead(&self) -> bool;
}

#[derive(Debug)]
pub struct ButtleEnemys<T: ButtleEnemysItem> {
    current_wave_idx: usize,
    inner: Vec<Vec<T>>,
}

impl<T: ButtleEnemysItem> ButtleEnemys<T> {
    pub(crate) fn new(enemy_data: &[Vec<EnemyData>]) -> Self {
        let waves = enemy_data
            .iter()
            .enumerate()
            .map(|(w, wave)| {
                wave.iter()
                    .enumerate()
                    .map(|(i, enemy)| {
                        T::new(
                            enemy,
                            RuntimeEnemyId {
                                wave_idx: w,
                                idx: i,
                            },
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(waves.len(), enemy_data.len());
        debug_assert!({
            waves
                .iter()
                .zip(enemy_data.iter())
                .all(|(a, b)| a.len() == b.len())
        });

        Self {
            current_wave_idx: 0,
            inner: waves,
        }
    }

    pub fn current_wave(&self) -> WaveNum {
        (self.current_wave_idx + 1) as WaveNum
    }

    pub fn get(&self, id: RuntimeEnemyId) -> &T {
        &self.inner[id.wave_idx][id.idx]
    }

    pub fn get_mut(&mut self, id: RuntimeEnemyId) -> &mut T {
        &mut self.inner[id.wave_idx][id.idx]
    }

    pub(crate) fn go_next_wave(&mut self) {
        assert!(!self.current_wave_is_last_wave());
        self.current_wave_idx += 1;
        assert!(self.current_wave_idx < self.inner.len());
    }

    pub(crate) fn current_wave_all_dead(&self) -> bool {
        self.inner[self.current_wave_idx]
            .iter()
            .all(|enemy| enemy.is_dead())
    }

    pub(crate) fn current_wave_is_last_wave(&self) -> bool {
        self.current_wave_idx + 1 == self.inner.len()
    }

    pub fn current_wave_enemys(&self) -> impl Iterator<Item = &T> {
        self.inner[self.current_wave_idx]
            .iter()
            .filter(|e| !e.is_dead())
    }
}

//--------------------------------------------------//
//                                                  //
//                       TEST                       //
//                                                  //
//--------------------------------------------------//
#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::EnemyData;

    // ---- テスト用の疑似敵 ----
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct MockEnemy {
        id: RuntimeEnemyId,
        dead: bool,
    }

    impl ButtleEnemysItem for MockEnemy {
        fn new(_data: &EnemyData, id: RuntimeEnemyId) -> Self {
            MockEnemy { id, dead: false }
        }

        fn is_dead(&self) -> bool {
            self.dead
        }
    }

    impl MockEnemy {
        fn kill(&mut self) {
            self.dead = true;
        }
    }

    #[test]
    fn test_get() {
        let data = vec![vec![EnemyData { level: 1 }, EnemyData { level: 1 }]];
        let mut enemys = ButtleEnemys::<MockEnemy>::new(&data);
        let id = RuntimeEnemyId {
            wave_idx: 0,
            idx: 1,
        };
        let enemy = enemys.get_mut(id);
        enemy.kill();
        assert!(enemys.get(id).dead);
    }

    #[test]
    fn test_current_wave_is_one() {
        let data = vec![vec![EnemyData { level: 1 }, EnemyData { level: 1 }]];

        let enemys = ButtleEnemys::<MockEnemy>::new(&data);

        assert_eq!(enemys.current_wave(), 1);
    }

    #[test]
    fn test_current_wave_all_dead() {
        let data = vec![vec![EnemyData { level: 1 }, EnemyData { level: 1 }]];

        let mut enemys = ButtleEnemys::<MockEnemy>::new(&data);

        // すべての敵を kill する
        for e in enemys.inner[0].iter_mut() {
            e.kill();
        }

        assert!(enemys.current_wave_all_dead());
    }

    #[test]
    fn test_go_next_wave() {
        let data = vec![vec![EnemyData { level: 1 }], vec![EnemyData { level: 1 }]];

        let mut enemys = ButtleEnemys::<MockEnemy>::new(&data);

        assert_eq!(enemys.current_wave(), 1);

        enemys.go_next_wave();

        assert_eq!(enemys.current_wave(), 2);
        assert!(enemys.current_wave_is_last_wave());
    }

    #[test]
    fn test_current_wave_enemys_filters_dead() {
        let data = vec![vec![EnemyData { level: 1 }, EnemyData { level: 1 }]];

        let mut enemys = ButtleEnemys::<MockEnemy>::new(&data);

        // 1体だけ倒す
        enemys.inner[0][0].dead = true;

        let alive: Vec<_> = enemys.current_wave_enemys().collect();
        assert_eq!(alive.len(), 1);
    }
}
