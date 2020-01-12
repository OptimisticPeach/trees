use cgmath::{Vector3, Zero};
use android_base::{Drawable, Transform, ViewProj, Transforms};
use crate::shader::LightShader;
use opengl_graphics::{GlGraphics, Texture};
use graphics::Context;
use crate::fs::load as fs_load;
use image::DynamicImage;
use image::png::PNGDecoder;
use texture::TextureSettings;

pub const POINTS_N_PER_VERTEX: &'static [[f32; 4]] = &[
    // -z
    [-1.0, -1.0, -1.0, 1.0],  // BLACK  //0
    [-1.0, 1.0, -1.0, 1.0],  // WHITE   //1
    [1.0, 1.0, -1.0, 1.0],  // RED      //2

    [-1.0, -1.0, -1.0, 1.0],  // BLACK  //0
    [1.0, 1.0, -1.0, 1.0],  // RED      //2
    [1.0, -1.0, -1.0, 1.0],  // BLUE    //3
    // -y
    [-1.0, -1.0, 1.0, 1.0],  // GREEN   //4
    [-1.0, -1.0, -1.0, 1.0],  // BLACK  //0
    [1.0, -1.0, -1.0, 1.0],  // BLUE    //3

    [-1.0, -1.0, 1.0, 1.0],  // GREEN   //4
    [1.0, -1.0, -1.0, 1.0],  // BLUE    //3
    [1.0, -1.0, 1.0, 1.0],  // CYAN     //7
    // -x
    [-1.0, -1.0, 1.0, 1.0],  // GREEN   //4
    [-1.0, 1.0, 1.0, 1.0],  // YELLOW   //5
    [-1.0, 1.0, -1.0, 1.0],  // WHITE   //1

    [-1.0, -1.0, 1.0, 1.0],  // GREEN   //4
    [-1.0, 1.0, -1.0, 1.0],  // WHITE   //1
    [-1.0, -1.0, -1.0, 1.0],  // BLACK  //0
    // z
    [-1.0, 1.0, 1.0, 1.0],  // YELLOW   //5
    [-1.0, -1.0, 1.0, 1.0],  // GREEN   //4
    [1.0, 1.0, 1.0, 1.0],  // MAGENTA   //6

    [1.0, 1.0, 1.0, 1.0],  // MAGENTA   //6
    [-1.0, -1.0, 1.0, 1.0],  // GREEN   //4
    [1.0, -1.0, 1.0, 1.0],  // CYAN     //7
    // y
    [-1.0, 1.0, -1.0, 1.0],  // WHITE   //1
    [-1.0, 1.0, 1.0, 1.0],  // YELLOW   //5
    [1.0, 1.0, 1.0, 1.0],  // MAGENTA   //6

    [-1.0, 1.0, -1.0, 1.0],  // WHITE   //1
    [1.0, 1.0, 1.0, 1.0],  // MAGENTA   //6
    [1.0, 1.0, -1.0, 1.0],  // RED      //2
    // x
    [1.0, -1.0, -1.0, 1.0],  // BLUE    //3
    [1.0, 1.0, -1.0, 1.0],  // RED      //2
    [1.0, -1.0, 1.0, 1.0],  // CYAN     //7

    [1.0, 1.0, -1.0, 1.0],  // RED      //2
    [1.0, 1.0, 1.0, 1.0],  // MAGENTA   //6
    [1.0, -1.0, 1.0, 1.0],  // CYAN     //7
];
pub const UV: &'static [[f32; 2]] = &[
    [0.0, 1.0], // 0
    [0.0, 0.0], // 1
    [1.0, 0.0], // 2
    [1.0, 1.0], // 3
    [1.0, 0.0], // 4
    [1.0, 1.0], // 5
    [0.0, 1.0], // 6
    [0.0, 0.0], // 7
];

pub const UV_N_PER_VERTEX: &'static [[f32; 2]] = &[
    UV[0], UV[1], UV[2],
    UV[0], UV[2], UV[3],
    UV[4], UV[0], UV[3],
    UV[4], UV[3], UV[7],
    UV[4], UV[5], UV[1],
    UV[4], UV[1], UV[0],
    UV[5], UV[4], UV[6],
    UV[6], UV[4], UV[7],
    UV[1], UV[5], UV[6],
    UV[1], UV[6], UV[2],
    UV[3], UV[2], UV[7],
    UV[2], UV[6], UV[7]
];
pub const NORMALS_N_PER_VERTEX: &'static [[f32; 3]] = &[
    [0., 0., -1.], [0., 0., -1.], [0., 0., -1.],
    [0., 0., -1.], [0., 0., -1.], [0., 0., -1.],
    [0., -1., 0.], [0., -1., 0.], [0., -1., 0.],
    [0., -1., 0.], [0., -1., 0.], [0., -1., 0.],
    [-1., 0., 0.], [-1., 0., 0.], [-1., 0., 0.],
    [-1., 0., 0.], [-1., 0., 0.], [-1., 0., 0.],
    [0., 0., 1.], [0., 0., 1.], [0., 0., 1.],
    [0., 0., 1.], [0., 0., 1.], [0., 0., 1.],
    [0., 1., 0.], [0., 1., 0.], [0., 1., 0.],
    [0., 1., 0.], [0., 1., 0.], [0., 1., 0.],
    [1., 0., 0.], [1., 0., 0.], [1., 0., 0.],
    [1., 0., 0.], [1., 0., 0.], [1., 0., 0.],
];

pub const INDICES_N_PER_VERTEX: &'static [u16] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35
];
pub const COLOURS_N_PER_VERTEX: &'static [[f32; 4]] = &[
    [0.97, 0.84, 0.43, 1.]; 36
];

pub struct Cube {
    pub light: Vector3<f32>,
    transform: Transform,
    tex: Texture,
}

impl Cube {
    pub fn new() -> Self {
        let file = fs_load("texture.png").unwrap();
        let decoder = PNGDecoder::new(&file[..]).unwrap();
        let image = DynamicImage::from_decoder(decoder).unwrap();
        let texture = Texture::from_image(&image.to_rgba(), &TextureSettings::new().convert_gamma(true));
        Self {
            light: Vector3::zero(),
            tex: texture,
            transform: Transform::identity(),
        }
    }
}

impl Drawable for Cube {
    type Shader = LightShader;
    fn draw_with(
        &mut self,
        data: &mut LightShader,
        graphics: &mut GlGraphics,
        context: &Context,
        cache: &mut ViewProj,
        transforms: &mut Transforms
    ) {
        let lock = transforms.push_transform(self.transform.clone());
        data.set_eye(cache.eye());
        graphics.shader_draw(
            data,
            &context.draw_state,
            POINTS_N_PER_VERTEX,
            Some(INDICES_N_PER_VERTEX),
            Some((&self.tex, UV_N_PER_VERTEX)),
            Some(COLOURS_N_PER_VERTEX),
            Some(NORMALS_N_PER_VERTEX),
            |shader, gl| {
                shader.light_uni.set(gl, self.light.as_ref());
                shader.view_matrix_uni.set(gl, cache.view_ref());
                shader.world_matrix_uni.set(gl, lock.current().as_ref());
                shader.projection_matrix_uni.set(gl, cache.projection_ref());
                shader.eye_uni.set(gl, cache.eye().as_ref());
            });
    }
}
