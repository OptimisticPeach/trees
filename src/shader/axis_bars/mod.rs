use opengl_graphics::shader_utils::{Shader, DynamicAttribute, compile_shader};
use opengl_graphics::gl::types::GLuint;
use opengl_graphics::{gl, GlGraphics};
use opengl_graphics::GLSL;
use opengl_graphics::shader_uniforms::{ShaderUniform, SUMat4x4};
use cgmath::{Matrix4, SquareMatrix, Vector3, Rad};

const FRAGMENT_SOURCE: &'static str = include_str!("./xyz_fragment.glsl");
const VERTEX_SOURCE: &'static str = include_str!("./xyz_vertex.glsl");

pub struct Xyz {
    // Shader items
    vao: GLuint,
    vertex_shader: GLuint,
    fragment_shader: GLuint,
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
    // Uniforms for the above matrices and other items
    pub projection_matrix_uni: ShaderUniform<SUMat4x4>,
    pub world_matrix_uni: ShaderUniform<SUMat4x4>,
    pub view_matrix_uni: ShaderUniform<SUMat4x4>,
}

impl Drop for Xyz {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteProgram(self.program);
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.fragment_shader);
        }
    }
}

impl Xyz {
    pub fn set_eye(&mut self, value: Vector3<f32>) {
        self.view = Matrix4::from_translation(value);
    }
    pub fn rotate_eye(&mut self, value: Vector3<f32>) {
        self.view = self.view * Matrix4::from_angle_x(Rad(value.x)) * Matrix4::from_angle_y(Rad(value.y)) * Matrix4::from_angle_z(Rad(value.z));
    }
    pub fn set_light(&mut self, value: [f32; 3]) {
        self.pos_buffer[12] = [value[0] - 0.2, value[1] - 0.2, value[2], 1.0];
        self.pos_buffer[13] = [value[0] + 0.2, value[1] - 0.2, value[2], 1.0];
        self.pos_buffer[14] = [value[0] - 0.2, value[1] + 0.2, value[2], 1.0];
        self.pos_buffer[15] = [value[0] + 0.2, value[1] + 0.2, value[2], 1.0];
    }
}

impl Shader for Xyz {
    type Vertex = [f32; 4];
    fn new(_: GLSL, gl: Option<&mut GlGraphics>) -> Self {
        let gl = gl.unwrap();
        let vertex_shader_compiled = compile_shader(gl::VERTEX_SHADER, VERTEX_SOURCE).expect("Vertex shader error");
        let fragment_shader_compiled = compile_shader(gl::FRAGMENT_SHADER, FRAGMENT_SOURCE).expect("Fragment shader error");

        let program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader_compiled);
            gl::AttachShader(program, fragment_shader_compiled);
            program
        };

        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::LinkProgram(program);
        }

        let pos = DynamicAttribute::xyzw(program, "pos").unwrap();
        let color = DynamicAttribute::rgba(program, "color").unwrap();
        gl.use_program(program);
        let projection_matrix_uni = gl.get_uniform("projection").expect("Could not find projection uniform");
        projection_matrix_uni.set(gl, &[0.0; 16]);
        let world_matrix_uni = gl.get_uniform("model").expect("Could not find model uniform");
        world_matrix_uni.set(gl, &[0.0; 16]);
        let view_matrix_uni = gl.get_uniform("view").expect("Could not find view uniform");
        view_matrix_uni.set(gl, &[0.0; 16]);
        gl.clear_program();

        Self {
            // Shader items
            vao,
            vertex_shader: vertex_shader_compiled,
            fragment_shader: fragment_shader_compiled,
            program,
            // Per vertex items
            pos,
            color,
            pos_buffer:     vec![[0.0, -0.2, 0.0, 1.0],
                                 [6.0, -0.2, 0.0, 1.0],
                                 [0.0, 0.2, 0.0, 1.0],
                                 [6.0, 0.2, 0.0, 1.0],

                                 [-0.2, 0.0, 0.0, 1.0],
                                 [-0.2, 6.0, 0.0, 1.0],
                                 [0.2, 0.0, 0.0, 1.0],
                                 [0.2, 6.0, 0.0, 1.0],

                                 [0.0, -0.2, 0.0, 1.0],
                                 [0.0, -0.2, 6.0, 1.0],
                                 [0.0, 0.2, 0.0, 1.0],
                                 [0.0, 0.2, 6.0, 1.0],

                                 [0.0; 4],
                                 [0.0; 4],
                                 [0.0; 4],
                                 [0.0; 4]],
            color_buffer:   vec![
                [1., 0., 0., 1.],
                [1., 0., 0., 1.],
                [1., 0., 0., 1.],
                [1., 0., 0., 1.],

                [0., 1., 0., 1.],
                [0., 1., 0., 1.],
                [0., 1., 0., 1.],
                [0., 1., 0., 1.],

                [0., 0., 1., 1.],
                [0., 0., 1., 1.],
                [0., 0., 1., 1.],
                [0., 0., 1., 1.],

                [1.; 4],
                [1.; 4],
                [1.; 4],
                [1.; 4],
            ],
            // Indices and offset
            indices: vec![
                0, 2, 1, 2, 3, 1,
                4, 5, 7, 4, 7, 6,
                8, 11, 9, 8, 10, 11,
                12, 14, 15, 12, 15, 13,
            ],
            offset: 0,
            // Matrices and vectors
            world: Matrix4::identity(),
            view: Matrix4::identity(),
            projection: Matrix4::identity(),
            // Uniforms
            world_matrix_uni,
            view_matrix_uni,
            projection_matrix_uni,
        }
    }

    fn flush(&mut self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::Disable(gl::CULL_FACE);
//            gl::Enable(gl::DEPTH_TEST);
            self.color.bind_vao(self.vao);
            self.color.set(&self.color_buffer);
            self.pos.bind_vao(self.vao);
            self.pos.set(&self.pos_buffer);
            gl::LineWidth(5.0);
            gl::DrawElements(gl::TRIANGLES, 24, gl::UNSIGNED_SHORT, self.indices.as_ptr() as *const _);
            gl::BindVertexArray(0);
        }
    }

    fn program(&self) -> GLuint {
        self.program
    }

    fn offset(&mut self) -> &mut usize {
        &mut self.offset
    }

    fn pos_buffer(&mut self) -> &mut Vec<[f32; 4]> { &mut self.pos_buffer }
    fn colour_buffer(&mut self) -> Option<&mut Vec<[f32; 4]>> { None }
    fn uv_buffer(&mut self) -> Option<&mut Vec<[f32; 2]>> { None }
    fn index_buffer(&mut self) -> Option<&mut Vec<u16>> { None }
    fn normal_buffer(&mut self) -> Option<&mut Vec<[f32; 3]>> { None }
}
