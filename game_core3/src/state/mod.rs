use crate::{
    MpNum,
    buttle_enemy::ButtleEnemy,
    enemys::{ButtleEnemys, RuntimeEnemyId},
    event::Event,
    lt_common::LtCommon,
    state::chars::{ButtleChars, RuntimeCharId},
};

pub mod chars;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LtId {
    Enemy(RuntimeEnemyId),
    Char(RuntimeCharId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Player,
    Enemy,
}

#[derive(Debug)]
pub struct GameState {
    focused_enemy: Option<RuntimeEnemyId>,
    chars: ButtleChars,
    enemys: ButtleEnemys<ButtleEnemy>,
}

impl GameState {
    pub fn accept_event(&mut self, event: Event) {
        todo!()
    }

    pub fn chars(&self) -> &ButtleChars {
        &self.chars
    }

    pub fn enemys(&self) -> &ButtleEnemys<ButtleEnemy> {
        &self.enemys
    }

    pub fn focused_enemy(&self) -> Option<&ButtleEnemy> {
        self.focused_enemy.map(|id| self.enemys.get(id))
    }

    /// # Panic
    /// すでにゲームが終了している場合、パニックになる
    pub(crate) fn get_enemy_with_highest_target_priority(&self) -> &ButtleEnemy {
        assert!(!self.check_game_end().game_ended());
        self.enemys_sorted_by_target_priority()
            .next()
            .expect("ゲームが終了していない場合、必ず1体以上の敵がいる")
    }

    pub fn enemys_sorted_by_target_priority(&self) -> impl Iterator<Item = &ButtleEnemy> {
        // フォーカスされた敵が先頭でそれ以外は配列の順番通り
        let f = self.focused_enemy();

        // Optionをinto_iterするとSomeの場合は要素が追加され、Noneの場合は何も追加されない
        f.into_iter()
            .chain(self.enemys.current_wave_enemys().filter(move |enemy| {
                let is_not_focused_enemy = Some(enemy.runtime_id()) != f.map(|f| f.runtime_id());
                !enemy.lt().is_dead() && is_not_focused_enemy
            }))
    }

    pub fn get_lt(&self, lt_id: LtId) -> &LtCommon {
        match lt_id {
            LtId::Char(id) => self.chars().get_char(id).lt(),
            LtId::Enemy(id) => self.enemys.get(id).lt(),
        }
    }

    pub fn check_game_end(&self) -> CheckGameEndResult {
        if self.chars.chars().iter().any(|char| char.lt().is_dead()) {
            return CheckGameEndResult::Lose;
        }
        if self.enemys.current_wave_all_dead() {
            if self.enemys.current_wave_is_last_wave() {
                CheckGameEndResult::Win
            } else {
                CheckGameEndResult::GoNextWave
            }
        } else {
            CheckGameEndResult::None
        }
    }

    pub fn player_mp(&self) -> MpNum {
        todo!()
    }
}

// TODO: 適当な名前を考えて適当なファイルに書き直す
pub enum CheckGameEndResult {
    Win,
    GoNextWave,
    Lose,
    None,
}

impl CheckGameEndResult {
    pub(crate) fn game_ended(&self) -> bool {
        matches!(self, CheckGameEndResult::Lose | CheckGameEndResult::Win)
    }
}
