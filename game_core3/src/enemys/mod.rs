use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeEnemyId {
    wave_idx: usize,
    idx: usize,
}

pub trait ButtleEnemysItem<D> {
    fn new(data: &D, id: RuntimeEnemyId) -> Self;
    fn is_dead(&self) -> bool;
    fn runtime_id(&self) -> RuntimeEnemyId;
}

#[derive(Debug, Clone)]
pub struct ButtleEnemys<T, D>
where
    T: ButtleEnemysItem<D>,
{
    current_wave_idx: usize,
    inner: Vec<Vec<T>>,
    _marker: PhantomData<D>,
}

impl<T, D> ButtleEnemys<T, D>
where
    T: ButtleEnemysItem<D>,
{
    pub(crate) fn new(enemy_data: &[Vec<D>]) -> Self {
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
            _marker: PhantomData,
        }
    }

    pub fn current_wave(&self) -> usize {
        self.current_wave_idx + 1
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

    pub(crate) fn current_wave_enemys_with_check_living(&self) -> EnemysIntoIterWithCheckLiving {
        EnemysIntoIterWithCheckLiving::new(
            self.inner[self.current_wave_idx]
                .iter()
                .map(|char| char.runtime_id()),
        )
    }

    pub fn current_wave_enemys(&self) -> impl Iterator<Item = &T> {
        self.inner[self.current_wave_idx]
            .iter()
            .filter(|e| !e.is_dead())
    }
}

//--------------------------------------------------//
//                                                  //
//          ENEMYS ITER WITH CHECK LIVING           //
//                                                  //
//--------------------------------------------------//


/// イテレーターを回している最中に敵に対して可変な操作を行わなければならなかったので
/// 参照ではなくIDを所有権を持つイテレータを作成した
/// 行動順に敵を返す
pub(crate) struct EnemysIntoIterWithCheckLiving {
    inner: std::vec::IntoIter<RuntimeEnemyId>,
}
impl EnemysIntoIterWithCheckLiving {
    fn new(enemys: impl Iterator<Item = RuntimeEnemyId>) -> Self {
        let inner = enemys.collect::<Vec<_>>().into_iter();
        Self { inner }
    }

    pub(crate) fn next_living_enemy<'a, E: ButtleEnemysItem<D>, D>(
        &mut self,
        state: &'a ButtleEnemys<E, D>,
    ) -> Option<&'a E> {
        loop {
            let id = self.inner.next()?;
            let enemy = state.get(id);
            if !enemy.is_dead() {
                break Some(enemy);
            }
        }
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

    // ---- テスト用の疑似敵 ----
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct MockEnemy {
        id: RuntimeEnemyId,
        dead: bool,
    }

    impl ButtleEnemysItem<MockEnemyData> for MockEnemy {
        fn new(_data: &MockEnemyData, id: RuntimeEnemyId) -> Self {
            MockEnemy { id, dead: false }
        }

        fn is_dead(&self) -> bool {
            self.dead
        }
        fn runtime_id(&self) -> RuntimeEnemyId {
            self.id
        }
    }

    impl MockEnemy {
        fn kill(&mut self) {
            self.dead = true;
        }
    }

    struct MockEnemyData;
    #[test]
    fn test_get() {
        let data = vec![vec![MockEnemyData, MockEnemyData]];
        let mut enemys = ButtleEnemys::<MockEnemy, MockEnemyData>::new(&data);
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
        let data = vec![vec![MockEnemyData, MockEnemyData]];

        let enemys = ButtleEnemys::<MockEnemy, MockEnemyData>::new(&data);

        assert_eq!(enemys.current_wave(), 1);
    }

    #[test]
    fn test_current_wave_all_dead() {
        let data = vec![vec![MockEnemyData, MockEnemyData]];

        let mut enemys = ButtleEnemys::<MockEnemy, MockEnemyData>::new(&data);

        // すべての敵を kill する
        for e in enemys.inner[0].iter_mut() {
            e.kill();
        }

        assert!(enemys.current_wave_all_dead());
    }

    #[test]
    fn test_go_next_wave() {
        let data = vec![vec![MockEnemyData], vec![MockEnemyData]];

        let mut enemys = ButtleEnemys::<MockEnemy, MockEnemyData>::new(&data);

        assert_eq!(enemys.current_wave(), 1);

        enemys.go_next_wave();

        assert_eq!(enemys.current_wave(), 2);
        assert!(enemys.current_wave_is_last_wave());
    }

    #[test]
    fn test_current_wave_enemys_filters_dead() {
        let data = vec![vec![MockEnemyData, MockEnemyData]];

        let mut enemys = ButtleEnemys::<MockEnemy, MockEnemyData>::new(&data);

        // 1体だけ倒す
        enemys.inner[0][0].dead = true;

        let alive: Vec<_> = enemys.current_wave_enemys().collect();
        assert_eq!(alive.len(), 1);
    }

    #[test]
    fn test_next_living_enemy_returns_only_alive() {
        let data = vec![vec![MockEnemyData, MockEnemyData, MockEnemyData]];
        let mut enemys = ButtleEnemys::<MockEnemy, MockEnemyData>::new(&data);

        // 1番目と3番目の敵を kill
        enemys.inner[0][0].kill();
        enemys.inner[0][2].kill();

        let mut iter = enemys.current_wave_enemys_with_check_living();

        // 最初の生存敵は index=1
        let first = iter.next_living_enemy(&enemys).unwrap();
        assert_eq!(first.runtime_id().idx, 1);

        // 次は生存者なし
        assert!(iter.next_living_enemy(&enemys).is_none());
    }

    #[test]
    fn test_next_living_enemy_all_dead() {
        let data = vec![vec![MockEnemyData, MockEnemyData]];
        let mut enemys = ButtleEnemys::<MockEnemy, MockEnemyData>::new(&data);

        // 全て kill
        for e in enemys.inner[0].iter_mut() {
            e.kill();
        }

        let mut iter = enemys.current_wave_enemys_with_check_living();

        assert!(iter.next_living_enemy(&enemys).is_none());
    }

    #[test]
    fn test_next_living_enemy_order() {
        let data = vec![vec![
            MockEnemyData,
            MockEnemyData,
            MockEnemyData,
            MockEnemyData,
        ]];
        let mut enemys = ButtleEnemys::<MockEnemy, MockEnemyData>::new(&data);

        // 1と3だけ生かす
        enemys.inner[0][0].kill();
        enemys.inner[0][2].kill();

        let mut iter = enemys.current_wave_enemys_with_check_living();

        let r1 = iter.next_living_enemy(&enemys).unwrap();
        assert_eq!(r1.runtime_id().idx, 1);

        let r2 = iter.next_living_enemy(&enemys).unwrap();
        assert_eq!(r2.runtime_id().idx, 3);

        assert!(iter.next_living_enemy(&enemys).is_none());
    }

    #[test]
    fn test_next_living_enemy_uses_runtime_id() {
        let data = vec![vec![MockEnemyData, MockEnemyData]];
        let mut enemys = ButtleEnemys::<MockEnemy, MockEnemyData>::new(&data);

        // index=0 を kill → index=1 のみが生存
        enemys.inner[0][0].kill();

        let mut iter = enemys.current_wave_enemys_with_check_living();

        let enemy = iter.next_living_enemy(&enemys).unwrap();
        assert_eq!(enemy.runtime_id().wave_idx, 0);
        assert_eq!(enemy.runtime_id().idx, 1);
    }

    #[test]
    fn test_next_living_enemy_enemy_dies_during_iteration() {
        let data = vec![vec![MockEnemyData, MockEnemyData, MockEnemyData]];
        let mut enemys = ButtleEnemys::<MockEnemy, MockEnemyData>::new(&data);

        let mut iter = enemys.current_wave_enemys_with_check_living();

        // 最初の生存敵（0番）を取得
        let first = iter.next_living_enemy(&enemys).unwrap();
        assert_eq!(first.runtime_id().idx, 0);

        // イテレーション途中で1番を kill する
        enemys.inner[0][1].kill();

        // 次の生存敵は idx=2 のはず
        let second = iter.next_living_enemy(&enemys).unwrap();
        assert_eq!(second.runtime_id().idx, 2);

        // もう生存者はいない
        assert!(iter.next_living_enemy(&enemys).is_none());
    }
}
