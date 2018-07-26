extern crate specs;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
#[macro_use]
extern crate specs_derive;

mod components;
mod resources;
mod systems;

use components::rotation::Rotation;
use specs::Builder;
use components::position::Position;
use resources::graphics::GraphicsContext;
use systems::render::Renderer;
use specs::World;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use resources::event::RenderEvents;
use resources::time::DeltaTime;
use specs::DispatcherBuilder;
use std::ops::DerefMut;
use specs::RunNow;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let gl = GlGraphics::new(opengl);
    let mut world = World::new();
    world.register::<components::position::Position>();
    world.register::<components::rotation::Rotation>();
    world.register::<components::velocity::Velocity>();
    world.add_resource(DeltaTime { time: 0.01 });
    world.add_resource(RenderEvents {args: None});
    world.add_resource(GraphicsContext{ gl: Some(gl) });
    world.create_entity()
        .with(Position { x: 100.0, y: 100.0})
        .with(Rotation { theta: 0.0})
        .build();
    let mut renderer = Renderer;
    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        match event {
            Event::Loop(l) => {
                match l {
                    Loop::Render(r) => {
                        world.write_resource::<RenderEvents>().deref_mut().args = Some(r);
                        renderer.run_now(&world.res);
                    },
                    Loop::Update(u) => {
                        println!("TODO: dispatch update event");
                    },
                    Loop::Idle(i) => {
                        println!("TODO: dispatch idle event");
                    },
                    Loop::AfterRender(i) => {
                        println!("TODO: dispatch afterRender event");
                    }
                }
            },
            Event::Input(i) => {
                println!("TODO: dispatch input event");
            },
            Event::Custom(eventId, arc) => {
                println!("TODO: dispatch custom event");
            }
        }
    }
}