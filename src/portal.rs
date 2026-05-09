use std::env;

// pub fn game_frame(state: &mut state::State) {
//     let [mut pitch, mut yaw] = [0; 2];
//     let [mut dx, mut dy, mut dz] = [0; 3];
//     if state.input.keys.contains("w") {
//         pitch += 1;
//     }
//     if state.input.keys.contains("s") {
//         pitch -= 1;
//     }
//     if state.input.keys.contains("a") {
//         yaw -= 1;
//     }
//     if state.input.keys.contains("d") {
//         yaw += 1;
//     }
//     if state.input.keys.contains("up") {
//         dz += 1;
//     }
//     if state.input.keys.contains("down") {
//         dz -= 1;
//     }
//     if state.input.keys.contains("left") {
//         dx -= 1;
//     }
//     if state.input.keys.contains("right") {
//         dx += 1;
//     }
//     if state.input.keys.contains("f") {
//         dy -= 1;
//     }
//     if state.input.keys.contains("r") {
//         dy += 1;
//     }
//     state.camera.update_rotation(pitch as f32 * state.delta, yaw as f32 * state.delta);
//     state.camera.update_translation(
//         dx as f32 * state.delta,
//         dy as f32 * state.delta,
//         dz as f32 * state.delta,
//     );

//     state.camera.render(state.back_buffer.width(), state.back_buffer.height(), |px, py, ray| {
//         if let Some(hit) = state.voxels.cast(ray) {
//             *state.back_buffer.get_mut([px, py]) = hit.color;
//         }
//     });
// }

pub fn game_init() {
    unsafe { env::set_var("RUST_LOG", "debug") };
    env_logger::init();
}

pub fn game_cleanup() {
    log::info!("Application terminated successfully");
}
