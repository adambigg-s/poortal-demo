use crate::{
    render::{self, Raster, camera, ray::CastRay},
    state, text, voxel,
};

pub const WTITLE: &str = "Poortal Demo";
pub const WWIDTH: usize = 256;
pub const WHEIGHT: usize = 196;
pub const CLEAR_COLOR: u32 = pack_color(25, 25, 40);

pub const fn pack_color(r: u8, g: u8, b: u8) -> u32 {
    0xff_u32 << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
}

#[derive(Default)]
pub struct Portal {
    pub back_buffer: render::RenderTarget<u32>,
    pub text_writer: text::TextWriter<'static, u32>,
    pub camera: camera::Camera,
    pub voxels: voxel::Voxels,
}

impl state::Application for Portal {
    fn config() -> state::WindowState {
        state::WindowState {
            title: WTITLE,
            width: WWIDTH,
            height: WHEIGHT,
            clear_color: CLEAR_COLOR,
        }
    }

    fn setup(context: &state::WindowState) -> Self {
        let camera = camera::Camera::builder()
            .movespeed(13.0)
            .lookspeed(1.2)
            .fov(75.0)
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

        Self {
            back_buffer: render::RenderTarget::new([context.width, context.height]),
            text_writer: text::TextWriter::default_font(text::TextConfig::new_color(pack_color(
                200, 200, 180,
            ))),
            camera,
            voxels,
        }
    }

    fn frame(&mut self, context: &mut state::FrameData, pixels: &mut [u32]) {
        self.back_buffer.fill(CLEAR_COLOR);
        let [mut pitch, mut yaw] = [0; 2];
        let [mut dx, mut dy, mut dz] = [0; 3];
        if context.input.key("keyw") {
            pitch += 1;
        }
        if context.input.key("keys") {
            pitch -= 1;
        }
        if context.input.key("keya") {
            yaw -= 1;
        }
        if context.input.key("keyd") {
            yaw += 1;
        }
        if context.input.key("arrowup") {
            dz += 1;
        }
        if context.input.key("arrowdown") {
            dz -= 1;
        }
        if context.input.key("arrowleft") {
            dx -= 1;
        }
        if context.input.key("arrowright") {
            dx += 1;
        }
        if context.input.key("keyf") {
            dy -= 1;
        }
        if context.input.key("keyr") {
            dy += 1;
        }
        self.camera.update_rotation(pitch as f32 * context.dt, yaw as f32 * context.dt);
        self.camera.update_translation(
            dx as f32 * context.dt,
            dy as f32 * context.dt,
            dz as f32 * context.dt,
        );

        self.camera.render(self.back_buffer.width(), self.back_buffer.height(), |px, py, ray| {
            if let Some(hit) = self.voxels.cast(ray) {
                *self.back_buffer.get_mut([px, py]) = hit.color;
            }
        });

        pixels.copy_from_slice(&self.back_buffer);
    }
}
