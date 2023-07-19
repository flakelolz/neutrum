use std::ops::Add;

#[derive(Default, Debug, Clone, Copy)]
pub struct IntVector2D {
    pub x: i32,
    pub y: i32,
}

impl Add for IntVector2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
