use raylib::{
    prelude::{Color, RaylibDraw, RaylibDrawHandle, Rectangle, Vector2},
    texture::Texture2D,
};

use crate::game_state::GameState;

pub fn render_player(d: &mut RaylibDrawHandle, game_state: &GameState, texture: &Texture2D, player: usize) {
    let x_pos = game_state.state[player].context.physics.position.x as f32;
    let y_pos = game_state.state[player].context.physics.position.y as f32;
    let origin_x = texture.width as f32;
    let origin_y = texture.height as f32;
    d.draw_circle(texture.width, 10, 30.0, Color::RED);
    d.draw_texture_pro(
        texture,
        Rectangle::new(0.0, 0.0, 46.0, 46.0),
        Rectangle::new(x_pos, y_pos, 256.0, 256.0),
        Vector2::new(origin_x / 2.0, 0.0),
        0.0,
        Color::WHITE,
    )
}
