const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;
mod game;
mod game_state;
mod finite_state_machine;
mod components;
mod math;
mod systems;
mod rendering;
mod configs;
mod debug;

const MAX_ENTITIES: usize = 2;
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("NEUCH")
        .build();
    rl.set_target_fps(60);
    
    game::game_loop(&mut rl, thread, SCREEN_WIDTH, SCREEN_HEIGHT);
}
