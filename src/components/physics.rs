use crate::math::IntVector2D;

#[derive(Default, Debug, Copy, Clone)]
pub struct PhysicsComponent {
    pub position: IntVector2D,
    pub velocity: IntVector2D,
    pub acceleration: IntVector2D,
    pub facing_left: bool,
    pub facing_opponent: bool,
}
