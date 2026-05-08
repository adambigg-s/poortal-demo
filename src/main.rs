use std::{env, sync};

use crate::render::{Raster, camera, text};

pub mod mem;
pub mod render;
pub mod voxel;

pub struct WindowData<'d> {
    name: &'d str,
    width: usize,
    height: usize,
    back_clear: u32,
    ops: minifb::WindowOptions,
}

pub static WINDOW: sync::Mutex<sync::LazyLock<WindowData>> = {
    sync::Mutex::new(sync::LazyLock::new(|| WindowData {
        name: "Poortal Game",
        width: 256,
        height: 196,
        back_clear: 0xff_u32 << 24 | 25_u32 << 16 | 25_u32 << 8 | 40_u32,
        ops: minifb::WindowOptions {
            resize: false,
            scale: minifb::Scale::X4,
            ..Default::default()
        },
    }))
};

pub struct State {
    window: minifb::Window,
    back_buffer: render::RenderTarget<u32>,
    text_writer: text::TextWriter<'static, u32>,

    camera: camera::Camera,
    voxels: voxel::Voxels,
}

impl State {
    pub fn frame(&mut self) {
        self.back_buffer.fill(WINDOW.lock().unwrap().back_clear);
        self.text_writer.reset();
        self.text_writer.write_str(&mut self.back_buffer, "Poortal game test text");
        self.window
            .update_with_buffer(&self.back_buffer, self.back_buffer.width(), self.back_buffer.height())
            .unwrap();
    }
}

pub fn game_setup() -> State {
    let windata = WINDOW.try_lock().unwrap();
    let window = minifb::Window::new(windata.name, windata.width, windata.height, windata.ops).unwrap();
    let target = render::RenderTarget::<u32>::new([windata.width, windata.height]);
    let writer = render::text::TextWriter::default_font(render::text::TextConfig::new_color(u32::MAX));

    State {
        window,
        back_buffer: target,
        text_writer: writer,
        camera: camera::Camera::default(),
        voxels: voxel::Voxels {},
    }
}

pub fn init() {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
}

pub fn check_exit(state: &State) -> bool {
    state.window.is_key_down(minifb::Key::Escape)
}

pub fn cleanup() {
    log::info!("Game closed");
}

fn main() {
    init();
    let mut state = game_setup();
    while !check_exit(&state) {
        state.frame();
    }
    cleanup();
}
