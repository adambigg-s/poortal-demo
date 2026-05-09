use winit::event_loop;

pub mod mem;
pub mod portal;
pub mod render;
pub mod state;
pub mod text;
pub mod voxel;

fn main() {
    // portal::game_init();
    // let mut state = portal::game_setup();
    // while !portal::check_exit(&state) {
    //     state.handle_events();
    //     state.update_frame();
    // }
    // portal::game_cleanup();

    portal::game_init();
    let mut app = state::App::new();
    event_loop::EventLoop::new().unwrap().run_app(&mut app).unwrap();
    portal::game_cleanup();
}
