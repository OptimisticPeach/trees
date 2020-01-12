use android_base::{Drawable, ViewProj, Transforms, ShaderContext};
use opengl_graphics::GlGraphics;
use graphics::Context;
use crate::drawable::{Water, Cube, Axis};
use std::sync::{Arc, Mutex};
use cgmath::Vector3;
use crate::controls::Camera;

pub struct World {
    water: Water,
    cube: Cube,
    axis: Axis,
    light: Vector3<f32>,
    time: f32,
    camera: Arc<Mutex<Camera>>
}

impl World {
    pub fn new(cam: Arc<Mutex<Camera>>) -> Self {
        let mut this = Self {
            light: Vector3 { x: 0.0, y: 10.0, z: 0.0 },
            cube: Cube::new(),
            water: Water::create(70, 8.0),
            axis: Axis::new([0.0; 3]),
            time: 0.0,
            camera: cam
        };
        this.cube.light = this.light;
        this.water.light = this.light;
        this.axis.set_light(this.light.into());
        this
    }
    pub fn update(&mut self, new_time: f32) {
        self.time = new_time;
        self.water.time = new_time;
    }
    pub fn size_change(&mut self, size: &(usize, usize)) {
        self.camera.lock().unwrap().size(*size);
    }
}

impl Drawable for World {
    type Shader = ();
    fn draw_with(&mut self, _shader: &mut (), _graphics: &mut GlGraphics, _context: &Context, cache: &mut ViewProj, _transforms: &mut Transforms) {
        cache.view = self.camera.lock().unwrap().mat();
    }
    fn draw_children(&mut self, context: &mut ShaderContext) {
        context.draw(&mut self.cube);
        context.draw(&mut self.water);
        context.draw(&mut self.axis);
    }
}
