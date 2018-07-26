use resources::graphics::GraphicsContext;
use components::{position::Position, rotation::Rotation};
use resources::{time::DeltaTime, event::RenderEvents};
use opengl_graphics::{ GlGraphics };
use specs::{Read, Write, ReadStorage, System};
use std::ops::Deref;
use specs::Join;

/// App some comment
pub struct Renderer;

impl<'a> System<'a> for Renderer {
    type SystemData = (Write<'a, GraphicsContext>,
                       Read<'a, RenderEvents>,
                       ReadStorage<'a, Rotation>,
                       ReadStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        use graphics::*;
        let (mut gctx, evt, rot, pos) = data;

        match evt.args {
            Some(args) => {
                match gctx.gl {
                    Some(ref mut gl) => gl.draw(args.viewport(), |c, gl| {
                        // Clear the screen.
                        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
                        clear(GREEN, gl);

                        // Draw the entities.
                        for (rot, pos) in (&rot, &pos).join() {
                            const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
                            let square = rectangle::square(0.0, 0.0, 50.0);
                            let transform = c.transform
                                .trans(pos.x, pos.y)
                                .rot_rad(rot.theta)
                                .trans(-25.0, -25.0);
                            // Draw a box rotating around the middle of the screen.
                            rectangle(RED, square, transform, gl);
                        }
                    }),
                    _ => {}
                }
            },
            _ => {}
        }
    }
}
