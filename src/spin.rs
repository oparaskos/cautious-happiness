use specs::{WriteStorage, System};
mod game;
pub struct Rotator;

impl<'a> System<'a> for Rotator {
    type SystemData = (Read<'a, DeltaTime>,
                       WriteStorage<'a, rotation::Rotation>);

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (delta, rot) = data;

        for r in rot.join() {
            r += delta;
        }
    }
}