pub mod chars;
pub mod container_args;
pub mod enemy_ai;
pub mod error;
pub mod game_core_actor;
pub mod game_state;
pub mod lt;
pub mod screen_actor;
pub mod skills;


mod container;
mod damage;
mod passive;

type Num = f32;

#[cfg(test)]
mod tests;
