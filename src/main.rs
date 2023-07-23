// #![allow(unused)]
mod character_data;
mod components;
mod configs;
mod debug;
mod finite_state_machine;
mod game;
mod game_state;
mod math;
mod rendering;
mod systems;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;
const MAX_ENTITIES: usize = 3;
const MAX_STATES: usize = 10;
const MAX_DEVICES: usize = 2;
const GROUND: i32 = SCREEN_HEIGHT - 100;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Neutrum")
        .build();

    rl.set_target_fps(60);

    game::game_loop(&mut rl, thread);
}
