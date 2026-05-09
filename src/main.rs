use std::{env, time};

use crate::render::camera;

pub mod mem;
pub mod render;
pub mod state;
pub mod voxel;

pub fn init() {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
}

pub fn check_exit(state: &state::State) -> bool {
    state.input.keys.contains("escape")
}

pub fn cleanup() {
    log::info!("Application terminated successfully");
}

pub fn game_setup() -> state::State {
    let windata = state::WINDOW.try_lock().unwrap();
    let mut window = minifb::Window::new(windata.name, windata.width, windata.height, windata.ops).unwrap();
    window.set_target_fps(9999);
    let target = render::RenderTarget::<u32>::new([windata.width, windata.height]);
    let writer = render::text::TextWriter::default_font(render::text::TextConfig::new_color(u32::MAX));
    let camera = camera::Camera::builder().movespeed(1.0).lookspeed(1.0).fov(75.0).build();

    state::State {
        window,
        back_buffer: target,
        text_writer: writer,
        input: Default::default(),
        camera,
        voxels: voxel::Voxels::default(),
        time: time::Instant::now(),
        tick: 0,
    }
}

pub fn game_frame(state: &mut state::State) {
    let canvas = &mut state.back_buffer;
    *canvas.get_mut([100, 100]) = 0xffff00ff;
}

fn main() {
    init();
    let mut state = game_setup();
    while !check_exit(&state) {
        state.handle_events();
        state.frame();
    }
    cleanup();
}
