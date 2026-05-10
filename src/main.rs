pub mod mem;
pub mod portal;
pub mod render;
pub mod state;
pub mod text;
pub mod voxel;

fn main() {
    state::run::<portal::Portal>();
}
