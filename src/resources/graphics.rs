use opengl_graphics::GlGraphics;

#[derive(Default)]
pub struct GraphicsContext {
    pub gl: Option<GlGraphics>
}