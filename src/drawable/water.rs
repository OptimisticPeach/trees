use std::collections::HashMap;
use opengl_graphics::GlGraphics;
use cgmath::{Vector3, Rad, Matrix4, Zero};
use android_base::{Drawable, Transform, ViewProj, Transforms};
use crate::shader::WaterLight;
use graphics::Context;
use std::f32::consts::{PI, SQRT_2};

pub struct Water {
    points: Vec<[f32; 4]>,
    indices: Vec<u16>,
    colours: Vec<[f32; 4]>,
    radius: f32,
    pub light: Vector3<f32>,
    transform: Transform,
    pub time: f32
}

impl Water {
    pub fn create(width: usize, full_radius: f32) -> Self {
        let width = (width + (width % 3)) as isize;
        if width == 0 {
            panic!("Creating empty water mesh!");
        }
        let mut value_map = HashMap::<(isize, isize), u16>::new();
        let mut points = Vec::<[f32; 4]>::new();
        let (sin, cos) = (PI / 4.0).sin_cos();
        for i_idx in 0..width {
            let i = i_idx as f32 - (width as f32 / 2.0) + 0.5;
                for j_idx in 0..width {
                let j = j_idx as f32 - (width as f32 / 2.0) + 0.5;

                const X_SCL: f32 = ((SQRT_2 * 2.0) - 2.0) / SQRT_2;

                let point = [X_SCL * (i * cos - j * sin), (i * sin + j * cos)];
                let dist = (point[0] * point[0] + point[1] * point[1]).sqrt();
                if dist < full_radius {
                    value_map.insert((i_idx as isize, j_idx as isize), points.len() as u16);
                    points.push([point[0], 0.0, point[1], 1.0]);
                }
            }
        }
        let mut indices = Vec::new();
        for i in 0..width {
            for j in 0..width {
                if (i * width + j) % 3 == 0 {
                    if let Some(&a) = value_map.get(&(i, j)) {
                        let b = value_map.get(&(i + 1, j - 1)).cloned();
                        let c = value_map.get(&(i + 1, j)).cloned();
                        let d = value_map.get(&(i, j + 1)).cloned();
                        let e = value_map.get(&(i - 1, j + 1)).cloned();
                        let f = value_map.get(&(i - 1, j)).cloned();
                        let g = value_map.get(&(i, j - 1)).cloned();
                        if let (Some(g), Some(b)) = (g, b) {
                            indices.extend_from_slice(&[a, g, b]);
                        }
                        if let (Some(b), Some(c)) = (b, c) {
                            indices.extend_from_slice(&[a, b, c]);
                        }
                        if let (Some(c), Some(d)) = (c, d) {
                            indices.extend_from_slice(&[a, c, d]);
                        }
                        if let (Some(d), Some(e)) = (d, e) {
                            indices.extend_from_slice(&[a, d, e]);
                        }
                        if let (Some(e), Some(f)) = (e, f) {
                            indices.extend_from_slice(&[a, e, f]);
                        }
                        if let (Some(f), Some(g)) = (f, g) {
                            indices.extend_from_slice(&[a, f, g]);
                        }
                    }
                }
            }
        }
        let len = points.len();
        let mut transform = Transform::identity();
        transform.scale(2.0);
        Self {
            points,
            indices,
            radius: full_radius,
            colours: vec![[0.3, 0.89, 0.87, 0.1]; len],
            light: Vector3::zero(),
            time: 0.0,
            transform,
        }
    }
}

impl Drawable for Water {
    type Shader = WaterLight;
    fn draw_with(
        &mut self,
        data: &mut WaterLight,
        graphics: &mut GlGraphics,
        context: &Context,
        cache: &mut ViewProj,
        transforms: &mut Transforms
    ) {
        let lock = transforms.push_transform(self.transform.clone());
        let time = self.time * 0.05;
        let scaler = {
            Matrix4::from_scale(1.0 / (self.radius * 2.0)) *
                Matrix4::from_angle_x(Rad(PI / 2.0)) *
                Matrix4::from_angle_y(Rad(time)) *
                Matrix4::from_translation(Vector3::new(0.0, 0.0, -SQRT_2)) *
                Matrix4::from_angle_y(Rad(time)) *
                Matrix4::from_translation(Vector3::new(0.5, 0.5, 0.5))
        };
        data.set_eye(cache.eye());
        graphics.shader_draw(
            data,
            &context.draw_state,
            &self.points,
            Some(&self.indices),
            None,
            Some(&self.colours),
            None,
            |shader, gl| {
                shader.light_uni.set(gl, self.light.as_ref());
                shader.view_matrix_uni.set(gl, cache.view_ref());
                shader.world_matrix_uni.set(gl, lock.current().as_ref());
                shader.projection_matrix_uni.set(gl, cache.projection_ref());
                shader.eye_uni.set(gl, cache.eye().as_ref());
                shader.scaler_matrix_uni.set(gl, scaler.as_ref());
                shader.light_colour_uni.set(gl, &[0.8, 0.8, 0.8, 1.0]);
            }
        );
    }
}
