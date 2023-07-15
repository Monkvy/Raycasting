
pub struct VertexArray {
    gl: gl_dstruct::Gl,
    id: u32
}

impl VertexArray {
    pub fn new(gl: &gl_dstruct::Gl) -> Self {
        let mut id = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut id);
        }

        Self { gl: gl.clone(), id }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.id) }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteVertexArrays(1, &mut self.id) }
    }
}
