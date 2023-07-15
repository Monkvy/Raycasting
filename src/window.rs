use gl_dstruct::gl;

use crate::err;


pub struct Window {
    pub gl: gl_dstruct::Gl,

    // Declared last to drop last 
    // (to prevent gl error 1282 on glDelete)
    _sdl_gl_ctx: sdl2::video::GLContext,
    sdl_win: sdl2::video::Window,
    sdl_event_pump: sdl2::EventPump
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, err::Error> {
        let sdl = sdl2::init().map_err(|e| err::sdl_init(e))?;
        let video_subsystem = sdl.video().map_err(|e| err::sdl_init(e))?;

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        let window = video_subsystem.window(title, width, height).opengl().build()
            .map_err(|e| err::new(&e.to_string()))?;
        let event_pump = sdl.event_pump().map_err(|e| err::new(e))?;

        let gl_ctx = window.gl_create_context().map_err(|e| err::new(e))?;
        let gl = gl_dstruct::load_with(|s| window.subsystem().gl_get_proc_address(s) as _);

        unsafe {
            gl.Enable(gl::DEBUG_OUTPUT);
            gl.DebugMessageCallback(Some(err::gl_debug_callback), std::ptr::null());
            gl.Viewport(0, 0, width as i32, height as i32);
        }

        Ok(Self { gl, _sdl_gl_ctx: gl_ctx, sdl_win: window, sdl_event_pump: event_pump })
    }

    pub fn get_events(&mut self) -> sdl2::event::EventPollIterator {
        self.sdl_event_pump.poll_iter()
    }

    pub fn swap(&self) {
        self.sdl_win.gl_swap_window();
    }
}
