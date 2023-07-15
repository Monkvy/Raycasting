mod err;
mod render;
mod window;

use gl_dstruct::gl;
use render::{Program, Shader, ShaderType, ArrayBuffer, DrawUsage, Vertex, VertexArray};
use window::Window;


fn main() -> Result<(), err::Error> {
    let mut window = Window::new(800, 600, "Raytracing")?;
    let program = Program::from_shaders(&window.gl, &[
        Shader::from_source(&window.gl, include_str!("shaders/default.frag"), ShaderType::FragmentShader)?,
        Shader::from_source(&window.gl, include_str!("shaders/default.vert"), ShaderType::VertexShader)?
    ])?;

    let mut vbo = ArrayBuffer::<Vertex>::new(&window.gl, DrawUsage::Static);
    vbo.write_data(&[
        Vertex::new((-0.5, 0.5), (0., 1.)),     // Topleft
        Vertex::new((0.5, 0.5), (1., 1.)),      // Topright
        Vertex::new((-0.5, -0.5), (0., 0.)),    // Bottomleft


        Vertex::new((-0.5, -0.5), (0., 0.)),    // Bottomleft
        Vertex::new((0.5, 0.5), (1., 1.)),      // Topright
        Vertex::new((0.5, -0.5), (1., 0.)),     // Bottomright
    ]);

    let vao = VertexArray::new(&window.gl);
    vao.bind();
    Vertex::attrib_pointers(&window.gl, vao.get_id());


    let mut open = true;
    while open {
        for event in window.get_events() {
            match event {
                sdl2::event::Event::Quit { timestamp: _ } => open = false,
                _ => ()
            }
        }

        program.bind();
        vao.bind();
        unsafe { 
            window.gl.ClearColor(0., 0., 0., 1.);
            window.gl.Clear(gl::COLOR_BUFFER_BIT);
            window.gl.DrawArrays(gl::TRIANGLES, 0, 6)
        }

        window.swap();
    }


    Ok(())
}
