use graphics::math::Scalar;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Rotation {
    pub theta: Scalar
}