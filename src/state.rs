use std::{collections, num, rc, time};

use winit::{application, event::DeviceEvent, window};

pub const WTITLE: &str = "Poortal Demo";
pub const WWIDTH: usize = 256;
pub const WHEIGHT: usize = 196;
pub const CLEAR_COLOR: u32 = 0xff_u32 << 24 | 25_u32 << 16 | 25_u32 << 8 | 40_u32;

use crate::{
    render::{self, camera},
    text, voxel,
};

#[derive(Debug, Default)]
#[allow(clippy::large_enum_variant)]
pub enum AppPhase {
    #[default]
    Startup,
    Running {
        window: rc::Rc<window::Window>,
        state: State,
    },
}

#[derive(Debug, Default)]
pub struct App {
    pub phase: AppPhase,
    pub context: Option<softbuffer::Context<rc::Rc<window::Window>>>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
}

impl application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if matches!(self.phase, AppPhase::Startup) {
            let window = rc::Rc::new(
                event_loop
                    .create_window(
                        winit::window::WindowAttributes::default()
                            .with_title(WTITLE)
                            .with_inner_size(winit::dpi::PhysicalSize::new(WWIDTH as u32, WHEIGHT as u32))
                            .with_resizable(false),
                    )
                    .unwrap(),
            );
            let state = State::new(rc::Rc::clone(&window));

            *self = App {
                phase: AppPhase::Running { window, state },
                context: None,
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let AppPhase::Running { window, state } = &mut self.phase
        else {
            return;
        };
        if window_id != window.id() {
            return;
        }

        #[allow(unused_variables)]
        match event {
            | winit::event::WindowEvent::ActivationTokenDone { serial, token } => todo!(),
            | winit::event::WindowEvent::DroppedFile(path_buf) => todo!(),
            | winit::event::WindowEvent::HoveredFile(path_buf) => todo!(),
            | winit::event::WindowEvent::HoveredFileCancelled => todo!(),
            | winit::event::WindowEvent::Ime(ime) => todo!(),
            | winit::event::WindowEvent::MouseWheel { device_id, delta, phase } => todo!(),
            | winit::event::WindowEvent::PinchGesture { device_id, delta, phase } => todo!(),
            | winit::event::WindowEvent::PanGesture { device_id, delta, phase } => todo!(),
            | winit::event::WindowEvent::DoubleTapGesture { device_id } => todo!(),
            | winit::event::WindowEvent::RotationGesture { device_id, delta, phase } => todo!(),
            | winit::event::WindowEvent::TouchpadPressure { device_id, pressure, stage } => todo!(),
            | winit::event::WindowEvent::AxisMotion { device_id, axis, value } => todo!(),
            | winit::event::WindowEvent::Touch(touch) => todo!(),
            | winit::event::WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
            | winit::event::WindowEvent::ThemeChanged(theme) => todo!(),
            | winit::event::WindowEvent::Occluded(_) => todo!(),
            | winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            | winit::event::WindowEvent::Destroyed => {
                event_loop.exit();
            }
            | winit::event::WindowEvent::Focused(true) => {}
            | winit::event::WindowEvent::Focused(false) => {}
            | winit::event::WindowEvent::Resized(physical_size) => {}
            | winit::event::WindowEvent::ModifiersChanged(modifiers) => {}
            | winit::event::WindowEvent::MouseInput { device_id, state, button } => {}
            | winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                let winit::keyboard::PhysicalKey::Code(keycode) = event.physical_key
                else {
                    return;
                };
                let name = keycode_name(keycode);
                match event.state {
                    | winit::event::ElementState::Pressed => state.input.keys.insert(name),
                    | winit::event::ElementState::Released => state.input.keys.remove(name),
                };
            }
            | winit::event::WindowEvent::CursorMoved { device_id, position } => {
                log::info!("Mouse position: {:?}", position);
            }
            | winit::event::WindowEvent::CursorEntered { device_id } => {}
            | winit::event::WindowEvent::Moved(physical_position) => {}
            | winit::event::WindowEvent::CursorLeft { device_id } => {}
            | winit::event::WindowEvent::RedrawRequested => {
                state.update_frame(rc::Rc::clone(window));
            }
        }
    }

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let AppPhase::Running { window, state } = &mut self.phase
        else {
            return;
        };

        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event {
            let (x, y) = &mut state.input.dmouse;
            *x += dx as f32;
            *y += dy as f32;
        }
    }
}

#[derive(Default, Debug)]
pub struct Input {
    pub keys: collections::HashSet<&'static str>,
    pub dmouse: (f32, f32),
    pub front: bool,
}

#[derive(Debug)]
pub struct State {
    pub surface: softbuffer::Surface<rc::Rc<window::Window>, rc::Rc<window::Window>>,
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
    pub fn new(window: rc::Rc<window::Window>) -> Self {
        let ctx = softbuffer::Context::new(rc::Rc::clone(&window)).unwrap();

        let mut surface = softbuffer::Surface::new(&ctx, rc::Rc::clone(&window)).unwrap();
        surface
            .resize(
                num::NonZeroU32::new(WWIDTH as u32).unwrap(),
                num::NonZeroU32::new(WHEIGHT as u32).unwrap(),
            )
            .unwrap();

        let writer = text::TextWriter::default_font(text::TextConfig::new_color(u32::MAX));
        let back_buffer = render::RenderTarget::new([WWIDTH, WHEIGHT]);
        let camera = camera::Camera::builder()
            .movespeed(13.0)
            .lookspeed(1.2)
            .fov(90.0)
            .pos(glam::vec3(10.0, 10.0, 10.0))
            .build();

        let mut voxels = voxel::Voxels::new(32, 32, 32);
        voxels.clear();
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

        State {
            surface,
            back_buffer,
            text_writer: writer,
            input: Default::default(),
            camera,
            voxels,
            time: time::Instant::now(),
            tick: 0,
            delta: 0.0,
        }
    }

    pub fn update_frame(&mut self, window: rc::Rc<window::Window>) {
        log::info!("FPS: {:.2}", self.delta.recip());
        log::info!("Mouse Delta: {:?}", self.input.dmouse);

        self.delta = self.time.elapsed().as_secs_f32();
        self.time = time::Instant::now();
        self.tick += 1;

        self.back_buffer.fill(CLEAR_COLOR);

        let mut sb_buffer = self.surface.buffer_mut().unwrap();
        let src = &*self.back_buffer;
        for (dst, &src_px) in sb_buffer.iter_mut().zip(src.iter()) {
            *dst = src_px;
        }
        sb_buffer.present().unwrap();

        window.request_redraw();
    }
}

fn keycode_name(keycode: winit::keyboard::KeyCode) -> &'static str {
    match keycode {
        | winit::keyboard::KeyCode::Backquote => "backquote",
        | winit::keyboard::KeyCode::Backslash => "backslash",
        | winit::keyboard::KeyCode::BracketLeft => "bracketleft",
        | winit::keyboard::KeyCode::BracketRight => "bracketright",
        | winit::keyboard::KeyCode::Comma => "comma",
        | winit::keyboard::KeyCode::Digit0 => "digit0",
        | winit::keyboard::KeyCode::Digit1 => "digit1",
        | winit::keyboard::KeyCode::Digit2 => "digit2",
        | winit::keyboard::KeyCode::Digit3 => "digit3",
        | winit::keyboard::KeyCode::Digit4 => "digit4",
        | winit::keyboard::KeyCode::Digit5 => "digit5",
        | winit::keyboard::KeyCode::Digit6 => "digit6",
        | winit::keyboard::KeyCode::Digit7 => "digit7",
        | winit::keyboard::KeyCode::Digit8 => "digit8",
        | winit::keyboard::KeyCode::Digit9 => "digit9",
        | winit::keyboard::KeyCode::Equal => "equal",
        | winit::keyboard::KeyCode::IntlBackslash => "intlbackslash",
        | winit::keyboard::KeyCode::IntlRo => "intlro",
        | winit::keyboard::KeyCode::IntlYen => "intlyen",
        | winit::keyboard::KeyCode::KeyA => "keya",
        | winit::keyboard::KeyCode::KeyB => "keyb",
        | winit::keyboard::KeyCode::KeyC => "keyc",
        | winit::keyboard::KeyCode::KeyD => "keyd",
        | winit::keyboard::KeyCode::KeyE => "keye",
        | winit::keyboard::KeyCode::KeyF => "keyf",
        | winit::keyboard::KeyCode::KeyG => "keyg",
        | winit::keyboard::KeyCode::KeyH => "keyh",
        | winit::keyboard::KeyCode::KeyI => "keyi",
        | winit::keyboard::KeyCode::KeyJ => "keyj",
        | winit::keyboard::KeyCode::KeyK => "keyk",
        | winit::keyboard::KeyCode::KeyL => "keyl",
        | winit::keyboard::KeyCode::KeyM => "keym",
        | winit::keyboard::KeyCode::KeyN => "keyn",
        | winit::keyboard::KeyCode::KeyO => "keyo",
        | winit::keyboard::KeyCode::KeyP => "keyp",
        | winit::keyboard::KeyCode::KeyQ => "keyq",
        | winit::keyboard::KeyCode::KeyR => "keyr",
        | winit::keyboard::KeyCode::KeyS => "keys",
        | winit::keyboard::KeyCode::KeyT => "keyt",
        | winit::keyboard::KeyCode::KeyU => "keyu",
        | winit::keyboard::KeyCode::KeyV => "keyv",
        | winit::keyboard::KeyCode::KeyW => "keyw",
        | winit::keyboard::KeyCode::KeyX => "keyx",
        | winit::keyboard::KeyCode::KeyY => "keyy",
        | winit::keyboard::KeyCode::KeyZ => "keyz",
        | winit::keyboard::KeyCode::Minus => "minus",
        | winit::keyboard::KeyCode::Period => "period",
        | winit::keyboard::KeyCode::Quote => "quote",
        | winit::keyboard::KeyCode::Semicolon => "semicolon",
        | winit::keyboard::KeyCode::Slash => "slash",
        | winit::keyboard::KeyCode::AltLeft => "altleft",
        | winit::keyboard::KeyCode::AltRight => "altright",
        | winit::keyboard::KeyCode::Backspace => "backspace",
        | winit::keyboard::KeyCode::CapsLock => "capslock",
        | winit::keyboard::KeyCode::ContextMenu => "contextmenu",
        | winit::keyboard::KeyCode::ControlLeft => "controlleft",
        | winit::keyboard::KeyCode::ControlRight => "controlright",
        | winit::keyboard::KeyCode::Enter => "enter",
        | winit::keyboard::KeyCode::SuperLeft => "superleft",
        | winit::keyboard::KeyCode::SuperRight => "superright",
        | winit::keyboard::KeyCode::ShiftLeft => "shiftleft",
        | winit::keyboard::KeyCode::ShiftRight => "shiftright",
        | winit::keyboard::KeyCode::Space => "space",
        | winit::keyboard::KeyCode::Tab => "tab",
        | winit::keyboard::KeyCode::Convert => "convert",
        | winit::keyboard::KeyCode::KanaMode => "kanamode",
        | winit::keyboard::KeyCode::Lang1 => "lang1",
        | winit::keyboard::KeyCode::Lang2 => "lang2",
        | winit::keyboard::KeyCode::Lang3 => "lang3",
        | winit::keyboard::KeyCode::Lang4 => "lang4",
        | winit::keyboard::KeyCode::Lang5 => "lang5",
        | winit::keyboard::KeyCode::NonConvert => "nonconvert",
        | winit::keyboard::KeyCode::Delete => "delete",
        | winit::keyboard::KeyCode::End => "end",
        | winit::keyboard::KeyCode::Help => "help",
        | winit::keyboard::KeyCode::Home => "home",
        | winit::keyboard::KeyCode::Insert => "insert",
        | winit::keyboard::KeyCode::PageDown => "pagedown",
        | winit::keyboard::KeyCode::PageUp => "pageup",
        | winit::keyboard::KeyCode::ArrowDown => "arrowdown",
        | winit::keyboard::KeyCode::ArrowLeft => "arrowleft",
        | winit::keyboard::KeyCode::ArrowRight => "arrowright",
        | winit::keyboard::KeyCode::ArrowUp => "arrowup",
        | winit::keyboard::KeyCode::NumLock => "numlock",
        | winit::keyboard::KeyCode::Numpad0 => "numpad0",
        | winit::keyboard::KeyCode::Numpad1 => "numpad1",
        | winit::keyboard::KeyCode::Numpad2 => "numpad2",
        | winit::keyboard::KeyCode::Numpad3 => "numpad3",
        | winit::keyboard::KeyCode::Numpad4 => "numpad4",
        | winit::keyboard::KeyCode::Numpad5 => "numpad5",
        | winit::keyboard::KeyCode::Numpad6 => "numpad6",
        | winit::keyboard::KeyCode::Numpad7 => "numpad7",
        | winit::keyboard::KeyCode::Numpad8 => "numpad8",
        | winit::keyboard::KeyCode::Numpad9 => "numpad9",
        | winit::keyboard::KeyCode::NumpadAdd => "numpadadd",
        | winit::keyboard::KeyCode::NumpadBackspace => "numpadbackspace",
        | winit::keyboard::KeyCode::NumpadClear => "numpadclear",
        | winit::keyboard::KeyCode::NumpadClearEntry => "numpadclearentry",
        | winit::keyboard::KeyCode::NumpadComma => "numpadcomma",
        | winit::keyboard::KeyCode::NumpadDecimal => "numpaddecimal",
        | winit::keyboard::KeyCode::NumpadDivide => "numpaddivide",
        | winit::keyboard::KeyCode::NumpadEnter => "numpadenter",
        | winit::keyboard::KeyCode::NumpadEqual => "numpadequal",
        | winit::keyboard::KeyCode::NumpadHash => "numpadhash",
        | winit::keyboard::KeyCode::NumpadMemoryAdd => "numpadmemoryadd",
        | winit::keyboard::KeyCode::NumpadMemoryClear => "numpadmemoryclear",
        | winit::keyboard::KeyCode::NumpadMemoryRecall => "numpadmemoryrecall",
        | winit::keyboard::KeyCode::NumpadMemoryStore => "numpadmemorystore",
        | winit::keyboard::KeyCode::NumpadMemorySubtract => "numpadmemorysubtract",
        | winit::keyboard::KeyCode::NumpadMultiply => "numpadmultiply",
        | winit::keyboard::KeyCode::NumpadParenLeft => "numpadparenleft",
        | winit::keyboard::KeyCode::NumpadParenRight => "numpadparenright",
        | winit::keyboard::KeyCode::NumpadStar => "numpadstar",
        | winit::keyboard::KeyCode::NumpadSubtract => "numpadsubtract",
        | winit::keyboard::KeyCode::Escape => "escape",
        | winit::keyboard::KeyCode::Fn => "fn",
        | winit::keyboard::KeyCode::FnLock => "fnlock",
        | winit::keyboard::KeyCode::PrintScreen => "printscreen",
        | winit::keyboard::KeyCode::ScrollLock => "scrolllock",
        | winit::keyboard::KeyCode::Pause => "pause",
        | winit::keyboard::KeyCode::BrowserBack => "browserback",
        | winit::keyboard::KeyCode::BrowserFavorites => "browserfavorites",
        | winit::keyboard::KeyCode::BrowserForward => "browserforward",
        | winit::keyboard::KeyCode::BrowserHome => "browserhome",
        | winit::keyboard::KeyCode::BrowserRefresh => "browserrefresh",
        | winit::keyboard::KeyCode::BrowserSearch => "browsersearch",
        | winit::keyboard::KeyCode::BrowserStop => "browserstop",
        | winit::keyboard::KeyCode::Eject => "eject",
        | winit::keyboard::KeyCode::LaunchApp1 => "launchapp1",
        | winit::keyboard::KeyCode::LaunchApp2 => "launchapp2",
        | winit::keyboard::KeyCode::LaunchMail => "launchmail",
        | winit::keyboard::KeyCode::MediaPlayPause => "mediaplaypause",
        | winit::keyboard::KeyCode::MediaSelect => "mediaselect",
        | winit::keyboard::KeyCode::MediaStop => "mediastop",
        | winit::keyboard::KeyCode::MediaTrackNext => "mediatracknext",
        | winit::keyboard::KeyCode::MediaTrackPrevious => "mediatrackprevious",
        | winit::keyboard::KeyCode::Power => "power",
        | winit::keyboard::KeyCode::Sleep => "sleep",
        | winit::keyboard::KeyCode::AudioVolumeDown => "audiovolumedown",
        | winit::keyboard::KeyCode::AudioVolumeMute => "audiovolumemute",
        | winit::keyboard::KeyCode::AudioVolumeUp => "audiovolumeup",
        | winit::keyboard::KeyCode::WakeUp => "wakeup",
        | winit::keyboard::KeyCode::Meta => "meta",
        | winit::keyboard::KeyCode::Hyper => "hyper",
        | winit::keyboard::KeyCode::Turbo => "turbo",
        | winit::keyboard::KeyCode::Abort => "abort",
        | winit::keyboard::KeyCode::Resume => "resume",
        | winit::keyboard::KeyCode::Suspend => "suspend",
        | winit::keyboard::KeyCode::Again => "again",
        | winit::keyboard::KeyCode::Copy => "copy",
        | winit::keyboard::KeyCode::Cut => "cut",
        | winit::keyboard::KeyCode::Find => "find",
        | winit::keyboard::KeyCode::Open => "open",
        | winit::keyboard::KeyCode::Paste => "paste",
        | winit::keyboard::KeyCode::Props => "props",
        | winit::keyboard::KeyCode::Select => "select",
        | winit::keyboard::KeyCode::Undo => "undo",
        | winit::keyboard::KeyCode::Hiragana => "hiragana",
        | winit::keyboard::KeyCode::Katakana => "katakana",
        | winit::keyboard::KeyCode::F1 => "f1",
        | winit::keyboard::KeyCode::F2 => "f2",
        | winit::keyboard::KeyCode::F3 => "f3",
        | winit::keyboard::KeyCode::F4 => "f4",
        | winit::keyboard::KeyCode::F5 => "f5",
        | winit::keyboard::KeyCode::F6 => "f6",
        | winit::keyboard::KeyCode::F7 => "f7",
        | winit::keyboard::KeyCode::F8 => "f8",
        | winit::keyboard::KeyCode::F9 => "f9",
        | winit::keyboard::KeyCode::F10 => "f10",
        | winit::keyboard::KeyCode::F11 => "f11",
        | winit::keyboard::KeyCode::F12 => "f12",
        | winit::keyboard::KeyCode::F13 => "f13",
        | winit::keyboard::KeyCode::F14 => "f14",
        | winit::keyboard::KeyCode::F15 => "f15",
        | winit::keyboard::KeyCode::F16 => "f16",
        | winit::keyboard::KeyCode::F17 => "f17",
        | winit::keyboard::KeyCode::F18 => "f18",
        | winit::keyboard::KeyCode::F19 => "f19",
        | winit::keyboard::KeyCode::F20 => "f20",
        | winit::keyboard::KeyCode::F21 => "f21",
        | winit::keyboard::KeyCode::F22 => "f22",
        | winit::keyboard::KeyCode::F23 => "f23",
        | winit::keyboard::KeyCode::F24 => "f24",
        | winit::keyboard::KeyCode::F25 => "f25",
        | winit::keyboard::KeyCode::F26 => "f26",
        | winit::keyboard::KeyCode::F27 => "f27",
        | winit::keyboard::KeyCode::F28 => "f28",
        | winit::keyboard::KeyCode::F29 => "f29",
        | winit::keyboard::KeyCode::F30 => "f30",
        | winit::keyboard::KeyCode::F31 => "f31",
        | winit::keyboard::KeyCode::F32 => "f32",
        | winit::keyboard::KeyCode::F33 => "f33",
        | winit::keyboard::KeyCode::F34 => "f34",
        | winit::keyboard::KeyCode::F35 => "f35",
        | _ => "",
    }
}
