use raylib::{
    prelude::{Color, RaylibDraw, RaylibDrawHandle, Rectangle, Vector2},
    texture::Texture2D,
};

use crate::{game_state::GameState, math::world_to_screen};

pub fn render_player(
    d: &mut RaylibDrawHandle,
    game_state: &GameState,
    texture: &Result<Texture2D, String>,
    player: usize,
) {
    let x_pos = world_to_screen(game_state.state[player].context.physics.position.x) as f32;
    let y_pos = world_to_screen(game_state.state[player].context.physics.position.y) as f32;
    let x_size = 256.0;
    let y_size = 256.0;

    match texture {
        Ok(texture) => {
            d.draw_texture_pro(
                texture,
                Rectangle::new(0.0, 0.0, 46.0, 46.0),
                Rectangle::new(x_pos, y_pos, x_size, y_size),
                Vector2::new(x_size / 2.0, y_size),
                0.0,
                Color::WHITE,
            )
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // d.draw_texture_pro(
    //     texture,
    //     Rectangle::new(0.0, 0.0, 46.0, 46.0),
    //     Rectangle::new(x_pos, y_pos, x_size, y_size),
    //     Vector2::new(x_size / 2.0, y_size),
    //     0.0,
    //     Color::WHITE,
    // )
}
