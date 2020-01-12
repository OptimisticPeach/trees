use android_base::{Drawable, ViewProj, Transforms};
use crate::shader::Xyz;
use opengl_graphics::GlGraphics;
use graphics::Context;

pub struct Axis([f32; 3]);

impl Drawable for Axis {
    type Shader = Xyz;
    fn draw_with(&mut self, shader: &mut Self::Shader, graphics: &mut GlGraphics, context: &Context, cache: &mut ViewProj, transforms: &mut Transforms) {
        shader.set_light(self.0);
        shader.set_eye(cache.eye());
        let lock = transforms.push_none();
        graphics.shader_draw(
            shader,
            &context.draw_state,
            &[],
            None,
            None,
            None,
            None,
            |shader, gl| {
                shader.view_matrix_uni.set(gl, cache.view_ref());
                shader.world_matrix_uni.set(gl, lock.current().as_ref());
                shader.projection_matrix_uni.set(gl, cache.projection_ref());
            }
        );
    }
}

impl Axis {
    pub fn new(light: [f32; 3]) -> Self {
        Axis(light)
    }
    pub fn set_light(&mut self, value: [f32; 3]) {
        self.0 = value;
    }
}
