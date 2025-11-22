use std::{borrow::Cow, collections::VecDeque, fmt::Debug};

use crate::{
    enemy_ai::EnemyAction,
    game_state::GameState,
    lt::{Char, Enemy},
};

// GameCoreActor以外の要素から何らかの情報を受け取る必要が無いようにする。
// 例えばターン終了などは入力関係を管理するActorから直接送ればいいからGameCoreActor
// とのインターフェースであるこのトレイトに実装する必要はない。
// けどそのやり方は混乱を招くと思うので避ける。
//
/// ## 概要
/// ScreenActorに対してメッセージを送信する要素
pub trait ScreenActorSender: Debug {
    fn initialize(&mut self, state: &GameState);
    fn update_char(&mut self, char: &Char);
    fn update_enemy(&mut self, enemy: &Enemy);
    fn start_player_turn(&mut self);
    fn end_player_turn(&mut self);
    fn update_player_mp(&mut self, mp: f32);
    fn update_enemy_mp(&mut self, mp: f32);
    fn update_enemy_actions(&mut self, actions: &VecDeque<EnemyAction>);
    fn log(&mut self, msg: Cow<'_, str>);
    fn win(&mut self);
    fn lose(&mut self);
}
