use graphics::math::Scalar;
use specs::VecStorage;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Rotation {
    pub theta: Scalar
}