
fn main() -> visim::Result<()> {
    let mut window = visim::Window::new("Raytracing", 1280, 720)?;

    while window.is_open() {
        window.get_events();
        window.update();
    }

    Ok(())
}
