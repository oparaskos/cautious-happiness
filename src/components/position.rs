use graphics::math::Scalar;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: Scalar,
    pub y: Scalar
}