use std::{
    borrow::Cow,
    collections::HashSet,
    ops::{Deref, DerefMut},
};

use rand::{rng, rngs::ThreadRng};

use crate::{
    Num,
    container_args::{CharData, ContainerArgs},
    enemy_ai::{EnemyAction, NUM_VIEW_ENEMY_ACTION},
    error::GameError,
    game_state::{GameResult, GameState},
    lt::{Char, Enemy},
    screen_actor::ScreenActorSender,
};

#[derive(Debug)]
pub struct Container {
    rng: ThreadRng,
    state: GameState,

    // MEMO: コンテナを引数にとる関数を書くたびに毎回毎回長いジェネリックを書かないといけないのが面倒だからimplではなくdynにした
    // 気合いで書き直す気になるかいい方法が見つかったらimplにしたほうが良い
    game_screen: Box<dyn ScreenActorSender>,
}

fn confrict_char(chars: &[CharData]) -> bool {
    let length = chars.len();
    let set = chars
        .iter()
        .map(|c| c.static_char_id)
        .collect::<HashSet<usize>>();

    length != set.len()
}

impl Container {
    pub fn new(
        data: &ContainerArgs,
        game_screen: Box<dyn ScreenActorSender>,
    ) -> Result<Self, GameError> {
        if confrict_char(&data.chars) {
            return Err(GameError::ConfrictChar);
        }

        let mut char_result_iter = data.chars.iter().map(Char::try_from);
        let contains_err = char_result_iter.clone().any(|x| x.is_err());
        let chars = if contains_err {
            return Err(char_result_iter.find(|x| x.is_err()).unwrap().unwrap_err());
        } else {
            char_result_iter.filter_map(|x| x.ok()).collect::<Vec<_>>()
        };

        let enemy = Enemy::try_from(&data.enemy)?;
        let mut rng = rng();
        let enemy_actions = (0..NUM_VIEW_ENEMY_ACTION)
            .map(|_| EnemyAction::random(&mut rng))
            .collect();

        Ok(Self {
            game_screen,
            rng,
            state: GameState {
                chars,
                enemy,
                player_side_mp: 0.0,
                enemy_side_mp: 0.0,
                enemy_actions,
                win_or_lose: GameResult::None,
            },
        })
    }
}

//----------------------------------------------------------------------------------------------------//
//                                                                                                    //
//                                                READ                                                //
//                                                                                                    //
//----------------------------------------------------------------------------------------------------//
impl Container {
    pub(crate) fn get_player_side_mp(&self) -> Num {
        self.state.player_side_mp
    }
    pub(crate) fn get_char(&self, static_char_id: usize) -> Result<&Char, GameError> {
        self.state
            .chars
            .iter()
            .find(|c| c.static_data.id == static_char_id)
            .ok_or(GameError::InvalidCharId)
    }
    pub(crate) fn get_enemy(&self) -> &Enemy {
        &self.state.enemy
    }
    pub(crate) fn current_turn_enemy_action(&self) -> EnemyAction {
        self.state.enemy_actions.front().copied().expect("msg")
    }

    pub(crate) fn get_chars(&self) -> &[Char] {
        assert!(!self.state.chars.is_empty());
        &self.state.chars
    }

    pub(crate) fn get_max_hate_char(&self) -> &Char {
        assert!(!self.state.chars.is_empty());

        self.state
            .chars
            .iter()
            .max_by(|a, b| a.hate.partial_cmp(&b.hate).unwrap())
            .expect("msg")
    }

    /// 現状保持している値を読むだけの関数
    ///
    /// 更新しない。ScreenActorに対して送信もしない.
    pub(crate) fn get_win_or_lose(&self) -> GameResult {
        self.state.win_or_lose
    }

    // MEMO: 入念なテストが必要そう
    pub(crate) fn check_and_send_win_or_lose(&mut self) -> GameResult {
        if self.state.win_or_lose != GameResult::None {
            return self.state.win_or_lose;
        }

        if self.state.chars.iter().any(|char| char.is_dead()) {
            self.game_screen.lose();
            self.state.win_or_lose = GameResult::Lose;
            return self.state.win_or_lose;
        };

        if self.state.enemy.is_dead() {
            self.game_screen.win();
            self.state.win_or_lose = GameResult::Win;
            return self.state.win_or_lose;
        };

        self.state.win_or_lose
    }

    pub(crate) fn get_state(&self) -> &GameState {
        &self.state
    }
}

//----------------------------------------------------------------------------------------------------//
//                                                                                                    //
//                                               WRITE                                                //
//                                                                                                    //
//----------------------------------------------------------------------------------------------------//

impl Container {
    pub(crate) fn set_turn_side(&mut self, player_side: bool) {
        if player_side {
            self.game_screen.start_player_turn();
        } else {
            self.game_screen.end_player_turn();
        }
    }
    // todo Cow
    pub(crate) fn log<'a>(&mut self, msg: impl Into<Cow<'a, str>>) {
        self.game_screen.log(msg.into());
    }

    pub(crate) fn heal_enemy_side_mp(&mut self, num: Num) {
        self.state.enemy_side_mp += num;
        self.game_screen.update_enemy_mp(self.state.enemy_side_mp);
    }

    pub(crate) fn heal_player_side_mp(&mut self, num: Num) {
        self.state.player_side_mp += num;
        self.game_screen.update_player_mp(self.state.player_side_mp);
    }

    fn get_mut_char(&mut self, static_char_id: usize) -> Result<CharMutRef<'_>, GameError> {
        let mut_ref = self
            .state
            .chars
            .iter_mut()
            .find(|c| c.static_data.id == static_char_id)
            .ok_or(GameError::InvalidCharId)?;

        Ok(CharMutRef {
            char: mut_ref,
            game_screen: self.game_screen.as_mut(),
        })
    }

    pub(crate) fn mut_chars_for_each(&mut self, update_fn: impl Fn(&mut Char)) {
        for char in self.state.chars.iter_mut() {
            update_fn(char);
            self.game_screen.update_char(char);
        }
    }

    // fn get_mut_enemy(&mut self) -> EnemyMutRef<'_> {
    //     EnemyMutRef {
    //         enemy: &mut self.state.enemy,
    //         game_screen: self.game_screen.as_mut(),
    //     }
    // }

    pub(crate) fn update_char(
        &mut self,
        static_char_id: usize,
        update_fn: impl FnOnce(&mut Char) -> Result<(), GameError>,
    ) -> Result<(), GameError> {
        let mut char = self.get_mut_char(static_char_id)?;
        update_fn(char.deref_mut())?;
        Ok(())
    }

    pub(crate) fn update_enemy(&mut self, update_fn: impl FnOnce(&mut Enemy)) {
        update_fn(&mut self.state.enemy);
        self.game_screen.update_enemy(&self.state.enemy);
    }

    pub(crate) fn consume_player_side_mp(&mut self, num: Num) {
        self.state.player_side_mp -= num;
        self.game_screen.update_player_mp(self.state.player_side_mp);
    }

    pub(crate) fn forword_enemy_action(&mut self) {
        let new = EnemyAction::random(&mut self.rng);
        self.state.enemy_actions.pop_front();
        self.state.enemy_actions.push_back(new);
        self.game_screen
            .update_enemy_actions(&self.state.enemy_actions);

        assert_eq!(self.state.enemy_actions.len(), NUM_VIEW_ENEMY_ACTION);
    }
}

//--------------------------------------------------//
//                   CHAR MUT REF                   //
//--------------------------------------------------//
pub struct CharMutRef<'a> {
    char: &'a mut Char,
    game_screen: &'a mut dyn ScreenActorSender,
}
impl<'a> Deref for CharMutRef<'a> {
    type Target = Char;
    fn deref(&self) -> &Self::Target {
        self.char
    }
}
impl<'a> DerefMut for CharMutRef<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.char
    }
}
impl<'a> Drop for CharMutRef<'a> {
    fn drop(&mut self) {
        self.game_screen.update_char(self.char);
    }
}

//--------------------------------------------------//
//                  ENEMY MUT REF                   //
//--------------------------------------------------//
// pub struct EnemyMutRef<'a> {
//     enemy: &'a mut Enemy,
//     game_screen: &'a mut dyn ScreenActorSender,
// }
// impl<'a> Deref for EnemyMutRef<'a> {
//     type Target = Enemy;
//     fn deref(&self) -> &Self::Target {
//         self.enemy
//     }
// }
// impl<'a> DerefMut for EnemyMutRef<'a> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.enemy
//     }
// }
// impl<'a> Drop for EnemyMutRef<'a> {
//     fn drop(&mut self) {
//         self.game_screen.update_enemy(self.enemy);
//     }
// }
