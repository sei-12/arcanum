use std::collections::VecDeque;

use crate::{
    Num,
    enemy_ai::EnemyAction,
    lt::{Char, Enemy},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameResult {
    None,
    Win,
    Lose,
}
impl GameResult {
    pub fn ended(&self) -> bool {
        self != &GameResult::None
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub win_or_lose: GameResult,
    pub chars: Vec<Char>,
    pub enemy: Enemy,
    pub player_side_mp: Num,
    pub enemy_side_mp: Num,
    pub enemy_actions: VecDeque<EnemyAction>,
}
