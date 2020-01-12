use android_base::{AppImpl, UpdateArgs, enable_backtrace, AppContainer, AppConfig, ShaderStorage, ShaderContext};
use graphics::{Context, clear};
use opengl_graphics::{GlGraphics, GLSL};
use piston::input::RenderArgs;
use crate::shader::{LightShader, Xyz, WaterLight, Texture3D};
use cgmath::{Rad, Vector3};
use std::f32::consts::FRAC_PI_4;
use std::sync::{Arc, Mutex};

mod shader;
mod fs;
mod drawable;
mod controls;
use noise::{OpenSimplex, NoiseFn};
use crate::controls::{Camera, spawn};
use crate::drawable::World;

pub struct App {
    world: World,
    time: f64,
}

impl AppImpl for App {
    type InitializationData = Arc<Mutex<Camera>>;
    fn new(gl: &mut GlGraphics, data: Self::InitializationData, shaders: &mut ShaderStorage) -> Self {
        shaders.cache.set_view_pos(Vector3::new(0., 0., 70.));
//        shaders.cache.rotate_view_axis_angle(Vector3::new(0., 1., 0.), PI / 2.0);
        shaders.get::<LightShader>(GLSL::V1_20, gl);
        shaders.get::<Xyz>(GLSL::V1_20, gl);
        let water_light = shaders.get::<WaterLight>(GLSL::V1_20, gl).0;
        const NOISE_SIZE: usize = 100;
        let perlin_data: Vec<u8> = {
            let mut data = vec![0u8; NOISE_SIZE * NOISE_SIZE * NOISE_SIZE];
            let noise = OpenSimplex::new();
            for i in 0..NOISE_SIZE {
                let i_val = i as f64;
                for j in 0..NOISE_SIZE {
                    let j_val = j as f64;
                    let offset = i + NOISE_SIZE * j;
                    for k in 0..NOISE_SIZE {
                        let k_val = k as f64;
                        let index = offset + NOISE_SIZE * NOISE_SIZE * k;
                        data[index] = (noise.get([i_val, j_val, k_val]) * 128.0 + 128.0) as u8;
                    }
                }
            }
            data
        };
        water_light.perlin = Texture3D::from_data(&perlin_data, [NOISE_SIZE; 3]).unwrap();
        Self {
            time: 0.0,
            world: World::new(data)
        }
    }

    fn on_size_change(&mut self, new: &(usize, usize), _old: &(usize, usize), shaders: &mut ShaderStorage) {
        println!("Projection initialized with {:?} as width/height", new);
        shaders.cache.set_projection(cgmath::perspective(Rad(FRAC_PI_4), new.0 as f32 / new.1 as f32, 0.1, 1000.0));
        self.world.size_change(new);
    }
    fn update(&mut self, args: UpdateArgs, _cfg: &mut AppConfig) {
        self.time += args.dt;
        self.world.update(self.time as _);
    }
    fn draw_shaded(&mut self, mut context: ShaderContext) {
        context.draw(&mut self.world);
    }
    fn draw_2d(&mut self, _c: Context, gl: &mut GlGraphics, args: RenderArgs, _cfg: &mut AppConfig) {
        self.time += args.ext_dt;
        clear([163.0 / 255.0, 250.0 / 255.0, 255.0 / 255.0, 1.], gl);
    }
    fn on_die(self) {
        println!("Dieing!");
    }
    fn cancel_poll(&self) -> bool {
        false
    }
}

pub fn main() {
    enable_backtrace();
    let camera = Arc::new(Mutex::new(Camera::default()));
    let cloned_camera = camera.clone();
    let mut container = AppContainer::<App>::init(AppConfig::new(), cloned_camera);
    spawn(&mut container, camera);
    container.run();
}
