use std::ffi::c_void;
use gl_dstruct::gl;


#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: (f32, f32),
    pub tex_coords: (f32, f32)
}

impl Vertex {
    pub fn new(pos: (f32, f32), tex_coords: (f32, f32)) -> Self {
        Self { pos, tex_coords }
    }

    pub fn attrib_pointers(gl: &gl_dstruct::Gl, vao_id: u32) {
        let self_size = std::mem::size_of::<Self>();
        unsafe {
            // pos
            gl.EnableVertexArrayAttrib(vao_id, 0);
            gl.VertexAttribPointer(
                0, 2, 
                gl::FLOAT, gl::FALSE,
                self_size as i32, std::ptr::null()
            );

            // tex_coords
            gl.EnableVertexArrayAttrib(vao_id, 1);
            gl.VertexAttribPointer(
                1, 2,
                gl::FLOAT, gl::FALSE,
                self_size as i32, std::mem::size_of::<(f32, f32)>() as *const c_void
            );
        }
    }
}
