use std::{env, time};

use crate::render::{Raster, camera, ray::CastRay};

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
    let target = render::RenderTarget::new([windata.width, windata.height]);
    let writer = render::text::TextWriter::default_font(render::text::TextConfig::new_color(u32::MAX));
    let camera = camera::Camera::builder()
        .movespeed(0.25)
        .lookspeed(1.0)
        .fov(75.0)
        .pos(glam::vec3(10.0, 10.0, 10.0))
        .build();

    let mut voxels = voxel::Voxels::new(32, 32, 32);
    for i in 0..32 {
        for j in 0..32 {
            if (i + j) % 2 == 0 {
                voxels.set(i, 31, j, 0xff00ffff);
            }
            else {
                voxels.set(i, 31, j, 0xffffff0f);
            }
        }
    }

    state::State {
        window,
        back_buffer: target,
        text_writer: writer,
        input: Default::default(),
        camera,
        voxels,
        time: time::Instant::now(),
        tick: 0,
        delta: 0.0,
    }
}

pub fn game_frame(state: &mut state::State) {
    let mut pitch = 0;
    let mut yaw = 0;
    let [mut dx, mut dy, mut dz] = [0, 0, 0];
    if state.input.keys.contains("w") {
        pitch += 1;
    }
    if state.input.keys.contains("s") {
        pitch -= 1;
    }
    if state.input.keys.contains("a") {
        yaw -= 1;
    }
    if state.input.keys.contains("d") {
        yaw += 1;
    }
    if state.input.keys.contains("up") {
        dz += 1;
    }
    if state.input.keys.contains("down") {
        dz -= 1;
    }
    if state.input.keys.contains("left") {
        dx -= 1;
    }
    if state.input.keys.contains("right") {
        dx += 1;
    }
    if state.input.keys.contains("f") {
        dy += 1;
    }
    if state.input.keys.contains("r") {
        dy -= 1;
    }
    state.camera.update_rotation(pitch as f32 * state.delta, yaw as f32 * state.delta);
    state.camera.update_translation(
        dx as f32 * state.delta,
        dy as f32 * state.delta,
        dz as f32 * state.delta,
    );

    state.camera.render(state.back_buffer.width(), state.back_buffer.height(), |px, py, ray| {
        if let Some(hit) = state.voxels.cast(ray) {
            *state.back_buffer.get_mut([px, py]) = hit.color;
        }
    });
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
