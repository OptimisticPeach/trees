use opengl_graphics::gl::types::GLuint;
use opengl_graphics::gl;

pub struct Texture3D {
    id: GLuint,
    width: u32,
    height: u32,
    depth: u32,
}

impl Texture3D {
    pub unsafe fn new(id: GLuint, width: u32, height: u32, depth: u32) -> Self {
        Self {
            id, width, height, depth
        }
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn empty() -> Result<Self, String> {
        Self::create(&[0u8; 4], [1, 1, 1])
    }

    pub fn from_data(data: &[u8], dimensions: [usize; 3]) -> Result<Self, String> {
        assert_eq!(data.len(), dimensions[0] * dimensions[1] * dimensions[2]);

        Self::create(data, dimensions)
    }

    fn create(memory: &[u8], size: [usize; 3]) -> Result<Self, String> {
        let mut id = 0;
        let internal_format = gl::R8;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_3D, id);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::MIRRORED_REPEAT as i32);
            gl::TexImage3D(
                gl::TEXTURE_3D,
                0,
                internal_format as i32,
                size[0] as i32,
                size[1] as i32,
                size[2] as i32,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                memory.as_ptr() as *const _);
            Ok(Self::new(id, size[0] as _, size[1] as _, size[2] as _))
        }
    }

    pub fn update_with(&mut self, data: &[u8]) {
        assert_eq!(data.len() as u32, self.width * self.height * self.depth);

        let offset = [0, 0, 0];
        unsafe {
            gl::BindTexture(gl::TEXTURE_3D, self.id);
            gl::TexSubImage3D(gl::TEXTURE_3D,
                              0,
                              offset[0],
                              offset[1],
                              offset[2],
                              self.width as _,
                              self.height as _,
                              self.depth as _,
                              gl::RED,
                              gl::UNSIGNED_BYTE,
                              data.as_ptr() as *const _);
        }
    }
}

impl Drop for Texture3D {
    fn drop(&mut self) {
        unsafe {
            let ids = [self.id];
            gl::DeleteTextures(1, ids.as_ptr());
        }
    }
}
