use raylib::prelude::*;

use crate::{
    character_data::Characters,
    configs::input_config::PlayerInput,
    debug::{debug_inputs::update_debug_inputs, debug_stats::show_debug_stats},
    game_state::GameState,
    math::{screen_to_world, IntVector2D},
    rendering::render_player,
    systems::{
        action_system::update_actions, collision_system::update_collision,
        input_system::update_inputs, physics_system::update_physics,
        reaction_system::update_reaction,
    },
    GROUND, SCREEN_WIDTH,
};
pub fn game_loop(rl: &mut RaylibHandle, thread: RaylibThread) {
    const P1: usize = 0;
    const P2: usize = 1;
    let texture = rl.load_texture(&thread, Characters::TestCharacter.sprite_path());
    let texture2 = rl.load_texture(&thread, Characters::TestCharacter2.sprite_path());
    let mut game_state = GameState::default();
    let mut input_config = PlayerInput::default();
    input_config.keybard[P2].set_p2();

    game_state.state[P1].processor.registry.init_states();
    game_state.state[P2].processor.registry.init_states();

    let world_ground = screen_to_world(GROUND);

    // Initial position of player
    game_state.state[P1].context.physics.position = IntVector2D {
        x: 420000,
        y: world_ground,
    };
    game_state.state[P2].context.physics.position = IntVector2D {
        x: 840000,
        y: world_ground,
    };

    let mut paused_game = false;


    while !rl.window_should_close() {
        let mut advance_once = false;
        // INPUTS
        update_debug_inputs(rl, &mut paused_game, &mut advance_once);
        update_inputs(rl, &mut game_state, &input_config);

        // GAME SIMULATION
        if !paused_game || advance_once {
            update_physics(&mut game_state);
            update_collision(&mut game_state);
            update_reaction(&mut game_state);
            update_actions(&mut game_state);
            game_state.frame_count += 1;
        }

        // RENDERING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        render_player(&mut d, &game_state, &texture, P1);
        render_player(&mut d, &game_state, &texture2, P2);

        // DEBUG
        show_debug_stats(&mut d, &mut game_state, P1);
        show_debug_stats(&mut d, &mut game_state, P2);
        if game_state.hit_events.len() > 0 {

            let trigger = &game_state.reaction_components[P2].hitstop.to_string();
            d.draw_text(trigger, 10, 100, 20, Color::WHITE);
        }


        if paused_game {
            d.draw_text("Paused", SCREEN_WIDTH / 2, 100, 20, Color::BEIGE);
        }
    }
}
