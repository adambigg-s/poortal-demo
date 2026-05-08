pub mod mem;
pub mod render;

fn main() {
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    log::debug!("Hello, World!");
}
