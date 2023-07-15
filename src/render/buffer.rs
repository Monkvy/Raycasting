use gl_dstruct::gl;



#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum DrawUsage {
    Static = 35044,
    Dynamic = 35048,
    Stream = 35040
}

pub struct ArrayBuffer<T> {
    gl: gl_dstruct::Gl,
    id: u32,
    usage: DrawUsage,
    _marker_t: std::marker::PhantomData<T>
}

impl<T> ArrayBuffer<T> {
    pub fn new(gl: &gl_dstruct::Gl, usage: DrawUsage) -> Self {
        let mut id = 0;
        unsafe { gl.GenBuffers(1, &mut id) }
        Self {
            gl: gl.clone(),
            id,
            usage,
            _marker_t: std::marker::PhantomData
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindBuffer(gl::ARRAY_BUFFER, self.id) }
    }

    pub fn write_data(&mut self, data: &[T]) {
        let data_size = data.len() * std::mem::size_of::<T>();

        self.bind();
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                data_size as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                self.usage as u32
            );
        }
    }
}

impl<T> Drop for ArrayBuffer<T> {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteBuffers(1, &mut self.id) }
    }
}
