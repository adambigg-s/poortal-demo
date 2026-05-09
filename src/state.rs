use std::{collections, sync, time};

use crate::{
    game_frame,
    render::{self, Raster, camera, text},
    voxel,
};

pub struct WindowData<'d> {
    pub name: &'d str,
    pub width: usize,
    pub height: usize,
    pub back_clear: u32,
    pub ops: minifb::WindowOptions,
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

#[derive(Default, Debug)]
pub struct Input {
    pub keys: collections::HashSet<&'static str>,
}

#[derive(Debug)]
pub struct State {
    pub window: minifb::Window,
    pub back_buffer: render::RenderTarget<u32>,
    pub text_writer: text::TextWriter<'static, u32>,

    pub input: Input,

    pub camera: camera::Camera,
    pub voxels: voxel::Voxels,

    pub time: time::Instant,
    pub tick: u64,
    pub delta: f32,
}

impl State {
    pub fn frame(&mut self) {
        self.delta = self.time.elapsed().as_secs_f32();
        log::info!("FPS: {:.2}", self.time.elapsed().as_secs_f32().recip());
        self.tick += 1;
        self.time = time::Instant::now();
        self.back_buffer.fill(WINDOW.lock().unwrap().back_clear);
        self.text_writer.reset();
        self.text_writer.write_str(&mut self.back_buffer, &format!("{:?}\n", self.input));
        self.text_writer.write_str(&mut self.back_buffer, &format!("{}", self.camera));
        game_frame(self);
        self.window
            .update_with_buffer(&self.back_buffer, self.back_buffer.width(), self.back_buffer.height())
            .unwrap();
    }

    pub fn handle_events(&mut self) {
        self.input.keys.clear();
        self.window.get_keys().iter().for_each(|key| {
            match key {
                | minifb::Key::Key0 => self.input.keys.insert("key0"),
                | minifb::Key::Key1 => self.input.keys.insert("key1"),
                | minifb::Key::Key2 => self.input.keys.insert("key2"),
                | minifb::Key::Key3 => self.input.keys.insert("key3"),
                | minifb::Key::Key4 => self.input.keys.insert("key4"),
                | minifb::Key::Key5 => self.input.keys.insert("key5"),
                | minifb::Key::Key6 => self.input.keys.insert("key6"),
                | minifb::Key::Key7 => self.input.keys.insert("key7"),
                | minifb::Key::Key8 => self.input.keys.insert("key8"),
                | minifb::Key::Key9 => self.input.keys.insert("key9"),
                | minifb::Key::A => self.input.keys.insert("a"),
                | minifb::Key::B => self.input.keys.insert("b"),
                | minifb::Key::C => self.input.keys.insert("c"),
                | minifb::Key::D => self.input.keys.insert("d"),
                | minifb::Key::E => self.input.keys.insert("e"),
                | minifb::Key::F => self.input.keys.insert("f"),
                | minifb::Key::G => self.input.keys.insert("g"),
                | minifb::Key::H => self.input.keys.insert("h"),
                | minifb::Key::I => self.input.keys.insert("i"),
                | minifb::Key::J => self.input.keys.insert("j"),
                | minifb::Key::K => self.input.keys.insert("k"),
                | minifb::Key::L => self.input.keys.insert("l"),
                | minifb::Key::M => self.input.keys.insert("m"),
                | minifb::Key::N => self.input.keys.insert("n"),
                | minifb::Key::O => self.input.keys.insert("o"),
                | minifb::Key::P => self.input.keys.insert("p"),
                | minifb::Key::Q => self.input.keys.insert("q"),
                | minifb::Key::R => self.input.keys.insert("r"),
                | minifb::Key::S => self.input.keys.insert("s"),
                | minifb::Key::T => self.input.keys.insert("t"),
                | minifb::Key::U => self.input.keys.insert("u"),
                | minifb::Key::V => self.input.keys.insert("v"),
                | minifb::Key::W => self.input.keys.insert("w"),
                | minifb::Key::X => self.input.keys.insert("x"),
                | minifb::Key::Y => self.input.keys.insert("y"),
                | minifb::Key::Z => self.input.keys.insert("z"),
                | minifb::Key::F1 => self.input.keys.insert("f1"),
                | minifb::Key::F2 => self.input.keys.insert("f2"),
                | minifb::Key::F3 => self.input.keys.insert("f3"),
                | minifb::Key::F4 => self.input.keys.insert("f4"),
                | minifb::Key::F5 => self.input.keys.insert("f5"),
                | minifb::Key::F6 => self.input.keys.insert("f6"),
                | minifb::Key::F7 => self.input.keys.insert("f7"),
                | minifb::Key::F8 => self.input.keys.insert("f8"),
                | minifb::Key::F9 => self.input.keys.insert("f9"),
                | minifb::Key::F10 => self.input.keys.insert("f10"),
                | minifb::Key::F11 => self.input.keys.insert("f11"),
                | minifb::Key::F12 => self.input.keys.insert("f12"),
                | minifb::Key::F13 => self.input.keys.insert("f13"),
                | minifb::Key::F14 => self.input.keys.insert("f14"),
                | minifb::Key::F15 => self.input.keys.insert("f15"),
                | minifb::Key::Down => self.input.keys.insert("down"),
                | minifb::Key::Left => self.input.keys.insert("left"),
                | minifb::Key::Right => self.input.keys.insert("right"),
                | minifb::Key::Up => self.input.keys.insert("up"),
                | minifb::Key::Apostrophe => self.input.keys.insert("apostrophe"),
                | minifb::Key::Backquote => self.input.keys.insert("backquote"),
                | minifb::Key::Backslash => self.input.keys.insert("backslash"),
                | minifb::Key::Comma => self.input.keys.insert("comma"),
                | minifb::Key::Equal => self.input.keys.insert("equal"),
                | minifb::Key::LeftBracket => self.input.keys.insert("leftbracket"),
                | minifb::Key::Minus => self.input.keys.insert("minus"),
                | minifb::Key::Period => self.input.keys.insert("period"),
                | minifb::Key::RightBracket => self.input.keys.insert("rightbracket"),
                | minifb::Key::Semicolon => self.input.keys.insert("semicolon"),
                | minifb::Key::Slash => self.input.keys.insert("slash"),
                | minifb::Key::Backspace => self.input.keys.insert("backspace"),
                | minifb::Key::Delete => self.input.keys.insert("delete"),
                | minifb::Key::End => self.input.keys.insert("end"),
                | minifb::Key::Enter => self.input.keys.insert("enter"),
                | minifb::Key::Escape => self.input.keys.insert("escape"),
                | minifb::Key::Home => self.input.keys.insert("home"),
                | minifb::Key::Insert => self.input.keys.insert("insert"),
                | minifb::Key::Menu => self.input.keys.insert("menu"),
                | minifb::Key::PageDown => self.input.keys.insert("pagedown"),
                | minifb::Key::PageUp => self.input.keys.insert("pageup"),
                | minifb::Key::Pause => self.input.keys.insert("pause"),
                | minifb::Key::Space => self.input.keys.insert("space"),
                | minifb::Key::Tab => self.input.keys.insert("tab"),
                | minifb::Key::NumLock => self.input.keys.insert("numlock"),
                | minifb::Key::CapsLock => self.input.keys.insert("capslock"),
                | minifb::Key::ScrollLock => self.input.keys.insert("scrolllock"),
                | minifb::Key::LeftShift => self.input.keys.insert("leftshift"),
                | minifb::Key::RightShift => self.input.keys.insert("rightshift"),
                | minifb::Key::LeftCtrl => self.input.keys.insert("leftctrl"),
                | minifb::Key::RightCtrl => self.input.keys.insert("rightctrl"),
                | minifb::Key::NumPad0 => self.input.keys.insert("numpad0"),
                | minifb::Key::NumPad1 => self.input.keys.insert("numpad1"),
                | minifb::Key::NumPad2 => self.input.keys.insert("numpad2"),
                | minifb::Key::NumPad3 => self.input.keys.insert("numpad3"),
                | minifb::Key::NumPad4 => self.input.keys.insert("numpad4"),
                | minifb::Key::NumPad5 => self.input.keys.insert("numpad5"),
                | minifb::Key::NumPad6 => self.input.keys.insert("numpad6"),
                | minifb::Key::NumPad7 => self.input.keys.insert("numpad7"),
                | minifb::Key::NumPad8 => self.input.keys.insert("numpad8"),
                | minifb::Key::NumPad9 => self.input.keys.insert("numpad9"),
                | minifb::Key::NumPadDot => self.input.keys.insert("numpaddot"),
                | minifb::Key::NumPadSlash => self.input.keys.insert("numpadslash"),
                | minifb::Key::NumPadAsterisk => self.input.keys.insert("numpadasterisk"),
                | minifb::Key::NumPadMinus => self.input.keys.insert("numpadminus"),
                | minifb::Key::NumPadPlus => self.input.keys.insert("numpadplus"),
                | minifb::Key::NumPadEnter => self.input.keys.insert("numpadenter"),
                | minifb::Key::LeftAlt => self.input.keys.insert("leftalt"),
                | minifb::Key::RightAlt => self.input.keys.insert("rightalt"),
                | minifb::Key::LeftSuper => self.input.keys.insert("leftsuper"),
                | minifb::Key::RightSuper => self.input.keys.insert("rightsuper"),
                | minifb::Key::Unknown => self.input.keys.insert("unknown"),
                | minifb::Key::Count => self.input.keys.insert("count"),
            };
        });
    }
}
