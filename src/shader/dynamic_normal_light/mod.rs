use opengl_graphics::shader_utils::{Shader, DynamicAttribute, compile_shader};
use opengl_graphics::gl::types::GLuint;
use opengl_graphics::{gl, GlGraphics};
use opengl_graphics::GLSL;
use graphics::BACK_END_MAX_VERTEX_COUNT;
use opengl_graphics::shader_uniforms::{ShaderUniform, SUMat4x4, SUVec3, SUVec4};
use cgmath::{Matrix4, SquareMatrix, Vector3, Rad};
use crate::shader::Texture3D;

const FRAGMENT_SOURCE: &'static str = include_str!("./fragment.glsl");
const VERTEX_SOURCE: &'static str = include_str!("./vertex.glsl");
const GEOMETRY_SOURCE: &'static str = include_str!("./geometry.glsl");
const CHUNKS: usize = 100;

pub struct WaterLight {
    // Shader items
    vao: GLuint,
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    geometry_shader: GLuint,
    program: GLuint,
    // Per-vertex attributes
    pos: DynamicAttribute,
    color: DynamicAttribute,
    // Per-vertex attribute buffers
    pos_buffer: Vec<[f32; 4]>,
    color_buffer: Vec<[f32; 4]>,
    // Indices and the offset
    indices: Vec<u16>,
    offset: usize,
    // Matrices and other items
    pub world: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub projection: Matrix4<f32>,
    pub eye: Vector3<f32>,
    // Uniforms for the above matrices and other items
    pub projection_matrix_uni: ShaderUniform<SUMat4x4>,
    pub world_matrix_uni: ShaderUniform<SUMat4x4>,
    pub view_matrix_uni: ShaderUniform<SUMat4x4>,
    pub light_uni: ShaderUniform<SUVec3>,
    pub eye_uni: ShaderUniform<SUVec3>,
    pub light_colour_uni: ShaderUniform<SUVec4>,
    pub scaler_matrix_uni: ShaderUniform<SUMat4x4>,
    // Water perlin noise texture data:
    pub perlin: Texture3D,
}

impl WaterLight {
    pub fn set_eye(&mut self, value: Vector3<f32>) {
        self.eye = value;
        self.view = Matrix4::from_translation(value);
    }
    pub fn rotate_eye(&mut self, value: Vector3<f32>) {
        self.view = self.view * Matrix4::from_angle_x(Rad(value.x)) * Matrix4::from_angle_y(Rad(value.y)) * Matrix4::from_angle_z(Rad(value.z));
    }
}

impl Drop for WaterLight {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteProgram(self.program);
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.geometry_shader);
            gl::DeleteShader(self.fragment_shader);
        }
    }
}

impl Shader for WaterLight {
    type Vertex = [f32; 4];
    fn new(_: GLSL, gl: Option<&mut GlGraphics>) -> Self {
        let gl = gl.unwrap();
        let vertex_shader_compiled = compile_shader(gl::VERTEX_SHADER, VERTEX_SOURCE).expect("Vertex shader error");
        let fragment_shader_compiled = compile_shader(gl::FRAGMENT_SHADER, FRAGMENT_SOURCE).expect("Fragment shader error");
        let geometry_shader_compiled = compile_shader(gl::GEOMETRY_SHADER, GEOMETRY_SOURCE).expect("Geometry shader error");
        let program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader_compiled);
            gl::AttachShader(program, geometry_shader_compiled);
            gl::AttachShader(program, fragment_shader_compiled);
            program
        };
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::LinkProgram(program);
        }

        let pos = DynamicAttribute::xyzw(program, "v_Pos").unwrap();
        let color = DynamicAttribute::rgba(program, "v_Color").unwrap();
        gl.use_program(program);
        let light_uni = gl.get_uniform("light").expect("Could not find light uniform");
        light_uni.set(gl, &[0.0; 3]);
        let projection_matrix_uni = gl.get_uniform("projection").expect("Could not find projection uniform");
        projection_matrix_uni.set(gl, &[0.0; 16]);
        let world_matrix_uni = gl.get_uniform("model").expect("Could not find model uniform");
        world_matrix_uni.set(gl, &[0.0; 16]);
        let view_matrix_uni = gl.get_uniform("view").expect("Could not find view uniform");
        view_matrix_uni.set(gl, &[0.0; 16]);
        let eye_uni = gl.get_uniform("eye").expect("Could not find eye uniform");
        eye_uni.set(gl, &[0.0; 3]);
        let light_colour_uni = gl.get_uniform("light_colour").expect("Could not find light colour uniform");
        light_colour_uni.set(gl, &[0.77, 0.61, 0.80, 1.0]);
        let scaler_matrix_uni = gl.get_uniform("texture_scaler").expect("Could not find texture scaler uniform");
        scaler_matrix_uni.set(gl, &[0.0; 16]);
        gl.clear_program();

        Self {
            // Shader items
            vao,
            vertex_shader: vertex_shader_compiled,
            geometry_shader: geometry_shader_compiled,
            fragment_shader: fragment_shader_compiled,
            program,
            // Per vertex items
            pos,
            color,
            pos_buffer: vec![[0.0; 4]; CHUNKS * BACK_END_MAX_VERTEX_COUNT],
            color_buffer: vec![[0.0; 4]; CHUNKS * BACK_END_MAX_VERTEX_COUNT],
            // Indices and offset
            indices: vec![0u16; 100],
            offset: 0,
            // Matrices and vectors
            world: Matrix4::from_translation(Vector3::new(0., 0., 100.)),
            view: Matrix4::identity(),
            projection: Matrix4::identity(),
            eye: Vector3::new(0., 0., 0.),
            // Uniforms
            world_matrix_uni,
            view_matrix_uni,
            projection_matrix_uni,
            light_uni,
            eye_uni,
            light_colour_uni,
            scaler_matrix_uni,
            perlin: Texture3D::empty().unwrap(),
        }
    }

    fn flush(&mut self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::Disable(gl::CULL_FACE);
            gl::BindTexture(gl::TEXTURE_3D, self.perlin.get_id());
//            gl::Enable(gl::DEPTH_TEST);
            self.color.bind_vao(self.vao);
            self.color.set(&self.color_buffer[..self.offset]);
            self.pos.bind_vao(self.vao);
            self.pos.set(&self.pos_buffer[..self.offset]);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_SHORT, self.indices.as_ptr() as *const _);
            gl::BindVertexArray(0);
            self.indices.clear();
        }

        self.offset = 0;
    }

    fn program(&self) -> GLuint {
        self.program
    }

    fn offset(&mut self) -> &mut usize {
        &mut self.offset
    }

    fn pos_buffer(&mut self) -> &mut Vec<[f32; 4]> {
        &mut self.pos_buffer
    }
    fn colour_buffer(&mut self) -> Option<&mut Vec<[f32; 4]>> {
        Some(&mut self.color_buffer)
    }
    fn uv_buffer(&mut self) -> Option<&mut Vec<[f32; 2]>> { None }
    fn index_buffer(&mut self) -> Option<&mut Vec<u16>> {
        Some(&mut self.indices)
    }
    fn normal_buffer(&mut self) -> Option<&mut Vec<[f32; 3]>> {
        None
    }
    fn texture_id(&mut self) -> Option<&mut GLuint> {
        None
    }
    fn has_texture(&self) -> bool {
        false
    }
}
