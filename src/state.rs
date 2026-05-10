use core::num;
use std::{collections, env, rc, time};

use winit::{application, dpi, event, event_loop, keyboard, window};

pub trait Application
where
    Self: Default,
{
    fn config() -> WindowState;

    fn setup(context: &WindowState) -> Self;

    fn frame(&mut self, context: &mut FrameData, pixels: &mut [u32]);
}

#[derive(Debug)]
pub struct WindowState {
    pub title: &'static str,
    pub width: usize,
    pub height: usize,
    pub clear_color: u32,
}

#[derive(Debug)]
pub struct FrameData<'r> {
    pub dt: f32,
    pub tick: u64,
    pub input: &'r Inputs,
    pub mouse_delta: (f32, f32),
}

#[derive(Default, Debug)]
pub struct Inputs {
    pub keys: collections::HashSet<&'static str>,
    pub dmouse: (f32, f32),
}

impl Inputs {
    pub fn key(&self, name: &str) -> bool {
        self.keys.contains(name)
    }

    pub fn consume_mouse(&mut self) -> (f32, f32) {
        let delta = self.dmouse;
        self.dmouse = Default::default();
        delta
    }
}

pub fn game_init() {
    unsafe { env::set_var("RUST_LOG", "debug") };
    env_logger::init();
}

pub fn game_cleanup() {
    log::info!("Application terminated successfully");
}

#[derive(Debug)]
pub struct State<T> {
    pub window: rc::Rc<window::Window>,
    pub surface: softbuffer::Surface<rc::Rc<window::Window>, rc::Rc<window::Window>>,
    pub inner_state: T,
    pub input: Inputs,
    pub time: time::Instant,
    pub delta: f32,
    pub tick: u64,
}

pub fn run<T>()
where
    T: Application + Default,
{
    game_init();
    event_loop::EventLoop::new().unwrap().run_app(&mut AppPhase::<T>::Startup).unwrap();
    game_cleanup();
}

#[derive(Default, Debug)]
pub enum AppPhase<T> {
    #[default]
    Startup,
    Running(State<T>),
}

impl<T> application::ApplicationHandler for AppPhase<T>
where
    T: Application,
{
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        if !matches!(self, AppPhase::Startup) {
            return;
        }

        let config = T::config();
        let window = rc::Rc::new(
            event_loop
                .create_window(
                    window::WindowAttributes::default()
                        .with_title(config.title)
                        .with_inner_size(dpi::PhysicalSize::new(config.width as u32, config.height as u32))
                        .with_resizable(false),
                )
                .unwrap(),
        );
        let context = softbuffer::Context::new(rc::Rc::clone(&window)).unwrap();
        let mut surface = softbuffer::Surface::new(&context, rc::Rc::clone(&window)).unwrap();
        surface
            .resize(
                num::NonZeroU32::new(config.width as u32).unwrap(),
                num::NonZeroU32::new(config.height as u32).unwrap(),
            )
            .unwrap();

        let app = T::setup(&config);

        *self = AppPhase::Running(State {
            window,
            surface,
            inner_state: app,
            input: Default::default(),
            time: time::Instant::now(),
            delta: Default::default(),
            tick: Default::default(),
        })
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        _: window::WindowId,
        event: event::WindowEvent,
    ) {
        let AppPhase::Running(state) = self
        else {
            return;
        };

        #[allow(unused_variables)]
        match event {
            | event::WindowEvent::ActivationTokenDone { serial, token } => todo!(),
            | event::WindowEvent::Moved(physical_position) => todo!(),
            | event::WindowEvent::DroppedFile(path_buf) => todo!(),
            | event::WindowEvent::HoveredFile(path_buf) => todo!(),
            | event::WindowEvent::HoveredFileCancelled => todo!(),
            | event::WindowEvent::ModifiersChanged(modifiers) => todo!(),
            | event::WindowEvent::Ime(ime) => todo!(),
            | event::WindowEvent::MouseWheel { device_id, delta, phase } => todo!(),
            | event::WindowEvent::MouseInput { device_id, state, button } => todo!(),
            | event::WindowEvent::PinchGesture { device_id, delta, phase } => todo!(),
            | event::WindowEvent::PanGesture { device_id, delta, phase } => todo!(),
            | event::WindowEvent::DoubleTapGesture { device_id } => todo!(),
            | event::WindowEvent::RotationGesture { device_id, delta, phase } => todo!(),
            | event::WindowEvent::TouchpadPressure { device_id, pressure, stage } => todo!(),
            | event::WindowEvent::AxisMotion { device_id, axis, value } => todo!(),
            | event::WindowEvent::Touch(touch) => todo!(),
            | event::WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
            | event::WindowEvent::ThemeChanged(theme) => todo!(),
            | event::WindowEvent::Occluded(_) => todo!(),
            | event::WindowEvent::CloseRequested => event_loop.exit(),
            | event::WindowEvent::Destroyed => event_loop.exit(),
            | event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                let keyboard::PhysicalKey::Code(keycode) = event.physical_key
                else {
                    return;
                };
                let name = keycode_name(keycode);
                if name == "escape" {
                    event_loop.exit();
                }
                match event.state {
                    | event::ElementState::Pressed => state.input.keys.insert(name),
                    | event::ElementState::Released => state.input.keys.remove(name),
                };
            }
            | event::WindowEvent::Focused(_) => {}
            | event::WindowEvent::Resized(_) => {}
            | event::WindowEvent::CursorMoved { .. } => {}
            | event::WindowEvent::CursorEntered { .. } => {}
            | event::WindowEvent::CursorLeft { .. } => {}
            | event::WindowEvent::RedrawRequested => {
                let now = time::Instant::now();
                state.delta = now.duration_since(state.time).as_secs_f32();
                state.time = now;
                state.tick += 1;

                let mouse_delta = state.input.consume_mouse();
                let config = T::config();
                let mut sb_buffer = state.surface.buffer_mut().unwrap();

                let mut context = FrameData {
                    dt: state.delta,
                    tick: state.tick,
                    input: &state.input,
                    mouse_delta,
                };

                state.inner_state.frame(&mut context, &mut sb_buffer);

                sb_buffer.present().unwrap();
                state.window.request_redraw();
            }
        }
    }
}

fn keycode_name(keycode: keyboard::KeyCode) -> &'static str {
    match keycode {
        | keyboard::KeyCode::Backquote => "backquote",
        | keyboard::KeyCode::Backslash => "backslash",
        | keyboard::KeyCode::BracketLeft => "bracketleft",
        | keyboard::KeyCode::BracketRight => "bracketright",
        | keyboard::KeyCode::Comma => "comma",
        | keyboard::KeyCode::Digit0 => "digit0",
        | keyboard::KeyCode::Digit1 => "digit1",
        | keyboard::KeyCode::Digit2 => "digit2",
        | keyboard::KeyCode::Digit3 => "digit3",
        | keyboard::KeyCode::Digit4 => "digit4",
        | keyboard::KeyCode::Digit5 => "digit5",
        | keyboard::KeyCode::Digit6 => "digit6",
        | keyboard::KeyCode::Digit7 => "digit7",
        | keyboard::KeyCode::Digit8 => "digit8",
        | keyboard::KeyCode::Digit9 => "digit9",
        | keyboard::KeyCode::Equal => "equal",
        | keyboard::KeyCode::IntlBackslash => "intlbackslash",
        | keyboard::KeyCode::IntlRo => "intlro",
        | keyboard::KeyCode::IntlYen => "intlyen",
        | keyboard::KeyCode::KeyA => "keya",
        | keyboard::KeyCode::KeyB => "keyb",
        | keyboard::KeyCode::KeyC => "keyc",
        | keyboard::KeyCode::KeyD => "keyd",
        | keyboard::KeyCode::KeyE => "keye",
        | keyboard::KeyCode::KeyF => "keyf",
        | keyboard::KeyCode::KeyG => "keyg",
        | keyboard::KeyCode::KeyH => "keyh",
        | keyboard::KeyCode::KeyI => "keyi",
        | keyboard::KeyCode::KeyJ => "keyj",
        | keyboard::KeyCode::KeyK => "keyk",
        | keyboard::KeyCode::KeyL => "keyl",
        | keyboard::KeyCode::KeyM => "keym",
        | keyboard::KeyCode::KeyN => "keyn",
        | keyboard::KeyCode::KeyO => "keyo",
        | keyboard::KeyCode::KeyP => "keyp",
        | keyboard::KeyCode::KeyQ => "keyq",
        | keyboard::KeyCode::KeyR => "keyr",
        | keyboard::KeyCode::KeyS => "keys",
        | keyboard::KeyCode::KeyT => "keyt",
        | keyboard::KeyCode::KeyU => "keyu",
        | keyboard::KeyCode::KeyV => "keyv",
        | keyboard::KeyCode::KeyW => "keyw",
        | keyboard::KeyCode::KeyX => "keyx",
        | keyboard::KeyCode::KeyY => "keyy",
        | keyboard::KeyCode::KeyZ => "keyz",
        | keyboard::KeyCode::Minus => "minus",
        | keyboard::KeyCode::Period => "period",
        | keyboard::KeyCode::Quote => "quote",
        | keyboard::KeyCode::Semicolon => "semicolon",
        | keyboard::KeyCode::Slash => "slash",
        | keyboard::KeyCode::AltLeft => "altleft",
        | keyboard::KeyCode::AltRight => "altright",
        | keyboard::KeyCode::Backspace => "backspace",
        | keyboard::KeyCode::CapsLock => "capslock",
        | keyboard::KeyCode::ContextMenu => "contextmenu",
        | keyboard::KeyCode::ControlLeft => "controlleft",
        | keyboard::KeyCode::ControlRight => "controlright",
        | keyboard::KeyCode::Enter => "enter",
        | keyboard::KeyCode::SuperLeft => "superleft",
        | keyboard::KeyCode::SuperRight => "superright",
        | keyboard::KeyCode::ShiftLeft => "shiftleft",
        | keyboard::KeyCode::ShiftRight => "shiftright",
        | keyboard::KeyCode::Space => "space",
        | keyboard::KeyCode::Tab => "tab",
        | keyboard::KeyCode::Convert => "convert",
        | keyboard::KeyCode::KanaMode => "kanamode",
        | keyboard::KeyCode::Lang1 => "lang1",
        | keyboard::KeyCode::Lang2 => "lang2",
        | keyboard::KeyCode::Lang3 => "lang3",
        | keyboard::KeyCode::Lang4 => "lang4",
        | keyboard::KeyCode::Lang5 => "lang5",
        | keyboard::KeyCode::NonConvert => "nonconvert",
        | keyboard::KeyCode::Delete => "delete",
        | keyboard::KeyCode::End => "end",
        | keyboard::KeyCode::Help => "help",
        | keyboard::KeyCode::Home => "home",
        | keyboard::KeyCode::Insert => "insert",
        | keyboard::KeyCode::PageDown => "pagedown",
        | keyboard::KeyCode::PageUp => "pageup",
        | keyboard::KeyCode::ArrowDown => "arrowdown",
        | keyboard::KeyCode::ArrowLeft => "arrowleft",
        | keyboard::KeyCode::ArrowRight => "arrowright",
        | keyboard::KeyCode::ArrowUp => "arrowup",
        | keyboard::KeyCode::NumLock => "numlock",
        | keyboard::KeyCode::Numpad0 => "numpad0",
        | keyboard::KeyCode::Numpad1 => "numpad1",
        | keyboard::KeyCode::Numpad2 => "numpad2",
        | keyboard::KeyCode::Numpad3 => "numpad3",
        | keyboard::KeyCode::Numpad4 => "numpad4",
        | keyboard::KeyCode::Numpad5 => "numpad5",
        | keyboard::KeyCode::Numpad6 => "numpad6",
        | keyboard::KeyCode::Numpad7 => "numpad7",
        | keyboard::KeyCode::Numpad8 => "numpad8",
        | keyboard::KeyCode::Numpad9 => "numpad9",
        | keyboard::KeyCode::NumpadAdd => "numpadadd",
        | keyboard::KeyCode::NumpadBackspace => "numpadbackspace",
        | keyboard::KeyCode::NumpadClear => "numpadclear",
        | keyboard::KeyCode::NumpadClearEntry => "numpadclearentry",
        | keyboard::KeyCode::NumpadComma => "numpadcomma",
        | keyboard::KeyCode::NumpadDecimal => "numpaddecimal",
        | keyboard::KeyCode::NumpadDivide => "numpaddivide",
        | keyboard::KeyCode::NumpadEnter => "numpadenter",
        | keyboard::KeyCode::NumpadEqual => "numpadequal",
        | keyboard::KeyCode::NumpadHash => "numpadhash",
        | keyboard::KeyCode::NumpadMemoryAdd => "numpadmemoryadd",
        | keyboard::KeyCode::NumpadMemoryClear => "numpadmemoryclear",
        | keyboard::KeyCode::NumpadMemoryRecall => "numpadmemoryrecall",
        | keyboard::KeyCode::NumpadMemoryStore => "numpadmemorystore",
        | keyboard::KeyCode::NumpadMemorySubtract => "numpadmemorysubtract",
        | keyboard::KeyCode::NumpadMultiply => "numpadmultiply",
        | keyboard::KeyCode::NumpadParenLeft => "numpadparenleft",
        | keyboard::KeyCode::NumpadParenRight => "numpadparenright",
        | keyboard::KeyCode::NumpadStar => "numpadstar",
        | keyboard::KeyCode::NumpadSubtract => "numpadsubtract",
        | keyboard::KeyCode::Escape => "escape",
        | keyboard::KeyCode::Fn => "fn",
        | keyboard::KeyCode::FnLock => "fnlock",
        | keyboard::KeyCode::PrintScreen => "printscreen",
        | keyboard::KeyCode::ScrollLock => "scrolllock",
        | keyboard::KeyCode::Pause => "pause",
        | keyboard::KeyCode::BrowserBack => "browserback",
        | keyboard::KeyCode::BrowserFavorites => "browserfavorites",
        | keyboard::KeyCode::BrowserForward => "browserforward",
        | keyboard::KeyCode::BrowserHome => "browserhome",
        | keyboard::KeyCode::BrowserRefresh => "browserrefresh",
        | keyboard::KeyCode::BrowserSearch => "browsersearch",
        | keyboard::KeyCode::BrowserStop => "browserstop",
        | keyboard::KeyCode::Eject => "eject",
        | keyboard::KeyCode::LaunchApp1 => "launchapp1",
        | keyboard::KeyCode::LaunchApp2 => "launchapp2",
        | keyboard::KeyCode::LaunchMail => "launchmail",
        | keyboard::KeyCode::MediaPlayPause => "mediaplaypause",
        | keyboard::KeyCode::MediaSelect => "mediaselect",
        | keyboard::KeyCode::MediaStop => "mediastop",
        | keyboard::KeyCode::MediaTrackNext => "mediatracknext",
        | keyboard::KeyCode::MediaTrackPrevious => "mediatrackprevious",
        | keyboard::KeyCode::Power => "power",
        | keyboard::KeyCode::Sleep => "sleep",
        | keyboard::KeyCode::AudioVolumeDown => "audiovolumedown",
        | keyboard::KeyCode::AudioVolumeMute => "audiovolumemute",
        | keyboard::KeyCode::AudioVolumeUp => "audiovolumeup",
        | keyboard::KeyCode::WakeUp => "wakeup",
        | keyboard::KeyCode::Meta => "meta",
        | keyboard::KeyCode::Hyper => "hyper",
        | keyboard::KeyCode::Turbo => "turbo",
        | keyboard::KeyCode::Abort => "abort",
        | keyboard::KeyCode::Resume => "resume",
        | keyboard::KeyCode::Suspend => "suspend",
        | keyboard::KeyCode::Again => "again",
        | keyboard::KeyCode::Copy => "copy",
        | keyboard::KeyCode::Cut => "cut",
        | keyboard::KeyCode::Find => "find",
        | keyboard::KeyCode::Open => "open",
        | keyboard::KeyCode::Paste => "paste",
        | keyboard::KeyCode::Props => "props",
        | keyboard::KeyCode::Select => "select",
        | keyboard::KeyCode::Undo => "undo",
        | keyboard::KeyCode::Hiragana => "hiragana",
        | keyboard::KeyCode::Katakana => "katakana",
        | keyboard::KeyCode::F1 => "f1",
        | keyboard::KeyCode::F2 => "f2",
        | keyboard::KeyCode::F3 => "f3",
        | keyboard::KeyCode::F4 => "f4",
        | keyboard::KeyCode::F5 => "f5",
        | keyboard::KeyCode::F6 => "f6",
        | keyboard::KeyCode::F7 => "f7",
        | keyboard::KeyCode::F8 => "f8",
        | keyboard::KeyCode::F9 => "f9",
        | keyboard::KeyCode::F10 => "f10",
        | keyboard::KeyCode::F11 => "f11",
        | keyboard::KeyCode::F12 => "f12",
        | keyboard::KeyCode::F13 => "f13",
        | keyboard::KeyCode::F14 => "f14",
        | keyboard::KeyCode::F15 => "f15",
        | keyboard::KeyCode::F16 => "f16",
        | keyboard::KeyCode::F17 => "f17",
        | keyboard::KeyCode::F18 => "f18",
        | keyboard::KeyCode::F19 => "f19",
        | keyboard::KeyCode::F20 => "f20",
        | keyboard::KeyCode::F21 => "f21",
        | keyboard::KeyCode::F22 => "f22",
        | keyboard::KeyCode::F23 => "f23",
        | keyboard::KeyCode::F24 => "f24",
        | keyboard::KeyCode::F25 => "f25",
        | keyboard::KeyCode::F26 => "f26",
        | keyboard::KeyCode::F27 => "f27",
        | keyboard::KeyCode::F28 => "f28",
        | keyboard::KeyCode::F29 => "f29",
        | keyboard::KeyCode::F30 => "f30",
        | keyboard::KeyCode::F31 => "f31",
        | keyboard::KeyCode::F32 => "f32",
        | keyboard::KeyCode::F33 => "f33",
        | keyboard::KeyCode::F34 => "f34",
        | keyboard::KeyCode::F35 => "f35",
        | _ => "",
    }
}
