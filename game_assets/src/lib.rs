use game_core7::{core_actor::CoreActor, state::GameStateArgs};


fn args() -> GameStateArgs {
    GameStateArgs {
        chars: vec![],
        enemys: vec![],
    }    
}

fn a(){
    let core = CoreActor::new(args()).unwrap();
}
