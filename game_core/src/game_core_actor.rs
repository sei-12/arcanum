use crate::{
    Num,
    chars::StaticCharId,
    container::Container,
    container_args::ContainerArgs,
    error::GameError,
    game_state::{GameResult, GameState},
    screen_actor::ScreenActorSender,
    skills::StaticSkillId,
};

#[derive(Debug)]
pub enum GameCoreActorCommand {
    GameStart {
        random_seed: u64,
    },
    PlayerTrunEnd,
    UseSkill {
        static_user_id: StaticCharId,
        static_skill_id: StaticSkillId,
    },
}

#[derive(Debug)]
pub struct GameCoreActor {
    container: Container,
}

//----------------------------------------------------------------------------------------------------//
//                                                                                                    //
//                                               PUBLIC                                               //
//                                                                                                    //
//----------------------------------------------------------------------------------------------------//
impl GameCoreActor {
    pub fn new(
        data: &ContainerArgs,
        screen: Box<dyn ScreenActorSender>,
    ) -> Result<Self, GameError> {
        let container = Container::new(data, screen)?;
        Ok(Self { container })
    }

    pub fn send(&mut self, command: GameCoreActorCommand) -> Result<GameResult, GameError> {
        match command {
            GameCoreActorCommand::GameStart { random_seed: _ } => game_start(&mut self.container),
            GameCoreActorCommand::PlayerTrunEnd => turn_end(&mut self.container),
            GameCoreActorCommand::UseSkill {
                static_user_id,
                static_skill_id,
            } => use_skill(&mut self.container, static_user_id, static_skill_id),
        }
    }

    pub fn get_state(&self) -> &GameState {
        self.container.get_state()
    }
}

fn start_player_turn(con: &mut Container) {
    con.log("あなたのターン");
    let add_heal_mp: Num = con.get_chars().iter().map(|c| c.add_heal_mp()).sum();
    con.heal_player_side_mp(100.0 + add_heal_mp);
    con.mut_chars_for_each(|char| {
        char.accept_turn_start_dmg();
        char.passive.trigger_turn_start();
        char.heal_skill_cooltime();
    });
    con.set_turn_side(true);
}

fn game_start(con: &mut Container) -> Result<GameResult, GameError> {
    if con.get_win_or_lose().ended() {
        return Err(GameError::GameEnded);
    }

    start_player_turn(con);

    let result = con.check_and_send_win_or_lose();
    Ok(result)
}

fn turn_end(con: &mut Container) -> Result<GameResult, GameError> {
    if con.get_win_or_lose().ended() {
        return Err(GameError::GameEnded);
    }

    // end player turn

    // start enemy turn
    con.log("敵のターン");
    con.heal_enemy_side_mp(100.0 + con.get_enemy().add_heal_mp());
    con.update_enemy(|enemy| {
        enemy.accept_turn_start_dmg();
        enemy.passive.trigger_turn_start();
    });

    if con.check_and_send_win_or_lose().ended() {
        return Ok(con.get_win_or_lose());
    };

    let action = con.current_turn_enemy_action();
    let enemy = con.get_enemy();
    enemy.static_data.play_action(action, con);

    if con.check_and_send_win_or_lose().ended() {
        return Ok(con.get_win_or_lose());
    };

    // end enemy turn
    con.forword_enemy_action();

    start_player_turn(con);

    let result = con.check_and_send_win_or_lose();
    Ok(result)
}

fn use_skill(
    con: &mut Container,
    static_user_id: usize,
    static_skill_id: StaticSkillId,
) -> Result<GameResult, GameError> {
    if con.get_win_or_lose().ended() {
        return Err(GameError::GameEnded);
    }

    let char = con.get_char(static_user_id)?;
    let skill = char.get_skill(static_skill_id)?;
    if con.get_player_side_mp() < skill.need_mp {
        return Err(GameError::NotEnoughMp);
    }

    (skill.call)(static_user_id, con)?;

    let result = con.check_and_send_win_or_lose();
    Ok(result)
}
