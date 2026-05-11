use crate::{
    camera,
    ray::CastRay,
    render::{self, Raster},
    state, text, voxel,
};

pub const WTITLE: &str = "Poortal Demo";
pub const WWIDTH: usize = 396;
pub const WHEIGHT: usize = 246;
pub const WSCALE: usize = 3;
pub const CLEAR_COLOR: u32 = pack_color(25, 25, 40);
pub const VSIZE: usize = 32;

pub const fn pack_color(r: u8, g: u8, b: u8) -> u32 {
    0xff_u32 << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
}

#[derive(Default)]
pub struct Portal {
    pub buffer: render::RenderTarget<u32>,
    pub writer: text::TextWriter<'static, u32>,
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
            scale: WSCALE,
        }
    }

    fn setup(context: &state::WindowState) -> Self {
        let camera = camera::Camera::builder()
            .movespeed(10.0)
            .lookspeed(0.005)
            .fov(67.0)
            .pos(glam::vec3(32.0 / 2.0, 32.0 / 2.0, 32.0 / 2.0))
            .renderdist(256.0)
            .build();
        let mut voxels = voxel::Voxels::new(VSIZE, VSIZE, VSIZE);
        voxels.clear();
        for i in 0..VSIZE {
            for j in 0..VSIZE {
                if (i + j) % 2 == 0 {
                    voxels.set(i, 0, j, pack_color(240, 240, 230));
                }
                else {
                    voxels.set(i, 0, j, pack_color(10, 10, 10));
                }
            }
        }

        Self {
            buffer: render::RenderTarget::new([context.width, context.height]),
            writer: text::TextWriter::default_font(text::TextConfig::new_color(pack_color(200, 200, 180))),
            camera,
            voxels,
        }
    }

    fn frame(&mut self, context: &mut state::FrameData, pixels: &mut [u32]) {
        log::info!("FPS: {}", context.dt.recip());

        self.buffer.fill(CLEAR_COLOR);

        // camera looking around
        let look = glam::Vec2::from(context.mouse_delta) * self.camera.lookspeed;
        self.camera.update_rotation(-look.y, -look.x);

        // camera translating around
        let [mut dx, mut dy, mut dz] = [0; 3];
        if context.input.key("keyw") {
            dz += 1;
        }
        if context.input.key("keys") {
            dz -= 1;
        }
        if context.input.key("keya") {
            dx -= 1;
        }
        if context.input.key("keyd") {
            dx += 1;
        }
        if context.input.key("shiftleft") {
            dy -= 1;
        }
        if context.input.key("space") {
            dy += 1;
        }
        self.camera.update_translation(
            dx as f32 * context.dt * self.camera.movespeed,
            dy as f32 * context.dt * self.camera.movespeed,
            dz as f32 * context.dt * self.camera.movespeed,
        );

        // camera render
        self.camera.render(self.buffer.width(), self.buffer.height(), |px, py, ray| {
            if let Some(hit) = self.voxels.cast(ray) {
                *self.buffer.get_mut([px, py]) = hit.color;
            }
        });

        self.writer.reset();
        self.writer.write_str(&mut self.buffer, &format!("FPS: {:.1?}", context.dt.recip()));

        // write back to window
        pixels.copy_from_slice(&self.buffer);
    }
}
