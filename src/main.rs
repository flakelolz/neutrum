const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;
mod game;
mod game_state;
mod finite_state_machine;
mod components;
mod math;
mod systems;
mod configs;
mod debug;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("NEUCH")
        .build();
    rl.set_target_fps(60);
    //
    game::game_loop(&mut rl, thread, SCREEN_WIDTH, SCREEN_HEIGHT);
}
