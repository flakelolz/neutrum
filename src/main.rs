// #![allow(unused)]
mod game;
mod game_state;
mod finite_state_machine;
mod components;
mod math;
mod systems;
mod rendering;
mod configs;
mod debug;
mod character_data;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;
const MAX_ENTITIES: usize = 3;
const MAX_STATES: usize = 10;
const MAX_DEVICES: usize = 2;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Neutrum")
        .build();
    rl.set_target_fps(60);
    
    game::game_loop(&mut rl, thread);
}
