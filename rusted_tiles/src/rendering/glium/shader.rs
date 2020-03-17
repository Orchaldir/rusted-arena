use glium::backend::Facade;
use std::fs;

pub fn load_program<F: Facade>(
    display: &F,
    vertex_file: &str,
    fragment_file: &str,
) -> glium::Program {
    let path = "resources/shader/";
    let vertex_shader =
        fs::read_to_string([path, vertex_file].concat()).expect("Could not load vertex shader");
    let fragment_shader =
        fs::read_to_string([path, fragment_file].concat()).expect("Could not load vertex shader");

    glium::Program::from_source(display, &vertex_shader, &fragment_shader, None).unwrap()
}
