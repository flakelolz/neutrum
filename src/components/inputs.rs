#[derive(Default, Debug, Copy, Clone)]
pub struct InputComponent {
    pub up: bool,
    pub down: bool,
    pub back: bool,
    pub forward: bool,
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub k: bool,
}
