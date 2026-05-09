use crate::mem::buffer;

pub trait Raster {
    type Item;

    fn size(&self) -> [usize; 2];

    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn get(&mut self, x: usize, y: usize) -> &mut Self::Item;

    fn peek(&mut self, x: usize, y: usize) -> &Self::Item;
}

pub type RenderTarget<T> = buffer::Buffer<T, 2>;

impl<T> Raster for RenderTarget<T> {
    type Item = T;

    fn size(&self) -> [usize; 2] {
        self.size()
    }

    fn width(&self) -> usize {
        self.size()[0]
    }

    fn height(&self) -> usize {
        self.size()[1]
    }

    fn get(&mut self, x: usize, y: usize) -> &mut Self::Item {
        RenderTarget::get_mut(self, [x, y])
    }

    fn peek(&mut self, x: usize, y: usize) -> &Self::Item {
        RenderTarget::get(self, [x, y])
    }
}

unsafe impl<T> Send for RenderTarget<T> {}

unsafe impl<T> Sync for RenderTarget<T> {}

pub mod text {
    use std::{
        fs,
        io::{self, Read},
    };

    use crate::render;

    const FONT_SIZE: usize = 256;

    static DEFAULT_FONT: Font = Font::from_bytes(include_bytes!("../res/font/FONT_8x8.bin"));

    type BinRune = u64;

    #[derive(Debug)]
    pub struct Font {
        pub runes: [BinRune; FONT_SIZE],
    }

    impl Font {
        pub fn load(path: &str) -> io::Result<Self> {
            let mut file = fs::File::open(path)?;
            let mut runes = [0u64; FONT_SIZE];
            let mut buffer = [0u8; 8];

            #[allow(clippy::needless_range_loop)]
            for i in 0..FONT_SIZE {
                if file.read_exact(&mut buffer).is_err() {
                    break;
                }
                runes[i] = u64::from_le_bytes(buffer);
            }

            Ok(Self { runes })
        }

        pub const fn from_bytes(bytes: &[u8]) -> Self {
            let mut runes = [0u64; FONT_SIZE];
            let mut i = 0;

            while i < FONT_SIZE {
                let mut buf = [0u8; 8];
                let mut j = 0;
                while j < 8 {
                    buf[j] = bytes[i * 8 + j];
                    j += 1;
                }
                runes[i] = u64::from_le_bytes(buf);
                i += 1;
            }

            Self { runes }
        }
    }

    impl Default for Font {
        fn default() -> Self {
            Self { runes: [0; 256] }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct TextConfig<T> {
        pub color: T,
        pub scale: usize,
        pub start_x: usize,
        pub start_y: usize,
        pub stride_x: usize,
        pub stride_y: usize,
    }

    impl<T> TextConfig<T>
    where
        T: Default,
    {
        pub fn new_color(color: T) -> Self {
            Self { color, ..Default::default() }
        }
    }

    impl<T> Default for TextConfig<T>
    where
        T: Default,
    {
        fn default() -> Self {
            Self {
                color: T::default(),
                scale: 1,
                start_x: 8,
                start_y: 8,
                stride_x: 8,
                stride_y: 10,
            }
        }
    }

    #[derive(Debug)]
    pub struct TextWriter<'d, T> {
        pub head_x: usize,
        pub head_y: usize,
        pub config: TextConfig<T>,
        pub font: &'d Font,
    }

    impl<'d, T> TextWriter<'d, T>
    where
        T: Copy,
    {
        pub fn new(font: &'d Font, config: TextConfig<T>) -> Self {
            Self {
                head_x: config.start_x,
                head_y: config.start_y,
                config,
                font,
            }
        }

        pub fn default_font(config: TextConfig<T>) -> Self {
            Self {
                head_x: config.start_x,
                head_y: config.start_y,
                config,
                font: &DEFAULT_FONT,
            }
        }

        pub fn reset(&mut self) {
            self.head_x = self.config.start_x;
            self.head_y = self.config.start_y;
        }

        pub fn newline(&mut self) {
            self.head_x = self.config.start_x;
            self.head_y += self.config.stride_y * self.config.scale;
        }

        pub fn set_pos(&mut self, x: usize, y: usize) {
            self.head_x = x;
            self.head_y = y;
        }

        pub fn write_char<R>(&mut self, raster: &mut R, chr: char)
        where
            R: render::Raster<Item = T>,
        {
            let scaled_stride_x = self.config.stride_x * self.config.scale;
            let scaled_stride_y = self.config.stride_y * self.config.scale;

            if self.head_x + scaled_stride_x >= raster.width() {
                self.newline();
            }

            if self.head_y + scaled_stride_y >= raster.height() {
                self.reset();
            }

            if chr == '\n' {
                self.newline();
                return;
            }

            let rune_idx = chr as usize;
            let rune = if rune_idx < self.font.runes.len() {
                self.font.runes[rune_idx]
            }
            else {
                0
            };

            let scale = self.config.scale;

            (0..8).for_each(|row| {
                (0..8).for_each(|col| {
                    let shift = 8 * row + (7 - col);

                    if (rune & (1 << shift)) != 0 {
                        (0..scale).for_each(|dy| {
                            (0..scale).for_each(|dx| {
                                let px = self.head_x + (col * scale) + dx;
                                let py = self.head_y + (row * scale) + dy;

                                if px < raster.width() && py < raster.height() {
                                    *raster.get(px, py) = self.config.color;
                                }
                            });
                        });
                    }
                });
            });

            self.head_x += scaled_stride_x;
        }

        pub fn write_str<R>(&mut self, raster: &mut R, text: &str)
        where
            R: render::Raster<Item = T>,
        {
            text.chars().for_each(|chr| {
                self.write_char(raster, chr);
            });
        }

        pub fn write_str_at<R>(&mut self, raster: &mut R, text: &str, col: usize, row: usize)
        where
            R: render::Raster<Item = T>,
        {
            self.set_pos(col, row);
            self.write_str(raster, text);
        }
    }

    impl<T> Default for TextWriter<'static, T>
    where
        T: Default,
    {
        fn default() -> Self {
            Self {
                head_x: Default::default(),
                head_y: Default::default(),
                config: Default::default(),
                font: &DEFAULT_FONT,
            }
        }
    }
}

pub mod ray {
    pub const EPS_RAY: f32 = 1e-6;
    pub const RAY_OFFSET: f32 = 1e-3;
    pub const REAL_INTERVAL: RayInterval = RayInterval { low: f32::EPSILON, high: f32::MAX };

    pub trait CastRay {
        fn cast(&self, ray: Ray) -> Option<RayHit>;
    }

    #[derive(Clone, Copy)]
    pub struct RayInterval {
        pub low: f32,
        pub high: f32,
    }

    impl RayInterval {
        pub fn surrounds(&self, value: f32) -> bool {
            (self.low..self.high).contains(&value)
        }
    }

    impl Default for RayInterval {
        fn default() -> Self {
            REAL_INTERVAL
        }
    }

    #[derive(Default, Clone, Copy)]
    pub struct Ray {
        pub origin: glam::Vec3,
        pub direction: glam::Vec3,
        pub tspan: RayInterval,
    }

    impl Ray {
        pub fn at(&self, time: f32) -> glam::Vec3 {
            self.origin + self.direction * time
        }
    }

    #[derive(Default, Clone, Copy)]
    pub struct RayHit {
        pub time: f32,
        pub color: u32,
    }
}

pub mod camera {
    use std::{f32, fmt};

    use crate::render::ray;

    pub const DEFAULT_RENDER_DISTANCE: f32 = 100.0;

    #[derive(Default, Debug)]
    pub struct CameraRays {
        origin: glam::Vec3,
        start: glam::Vec3,
        drdx: glam::Vec3,
        dudy: glam::Vec3,
    }

    impl CameraRays {
        pub fn new(camera: &Camera, width: usize, height: usize) -> Self {
            let hw = (camera.fov.to_radians() / 2.0).tan();
            let hh = hw * (height as f32 / width as f32);
            let drdx = camera.rvec * (2.0 * hw / width as f32);
            let dudy = camera.uvec * -(2.0 * hh / height as f32);
            let start = camera.fvec - camera.rvec * hw + camera.uvec * hh + drdx * 0.5 + dudy * 0.5;

            Self { origin: camera.pos, start, drdx, dudy }
        }

        pub fn point_to_pixel(&self, px: usize, py: usize) -> glam::Vec3 {
            (self.start + self.drdx * px as f32 + self.dudy * py as f32).normalize()
        }
    }

    #[derive(bon::Builder, Default, Debug)]
    pub struct Camera {
        #[builder(default)]
        pub fvec: glam::Vec3,
        #[builder(default)]
        pub rvec: glam::Vec3,
        #[builder(default)]
        pub uvec: glam::Vec3,
        #[builder(default = -glam::Vec3::Y)]
        pub wupvec: glam::Vec3,

        #[builder(default)]
        pub yaw: f32,
        #[builder(default)]
        pub pitch: f32,
        #[builder(default)]
        pub pos: glam::Vec3,

        pub lookspeed: f32,
        pub movespeed: f32,

        pub fov: f32,
        #[builder(default = DEFAULT_RENDER_DISTANCE)]
        pub renderdist: f32,
    }

    impl Camera {
        // TODO: this doesn't work at all, need to fix this
        // INFO: minifb input is really bad, no mouse support
        pub fn update_rotation(&mut self, dp: f32, dy: f32) {
            self.pitch += dp * self.lookspeed;
            self.yaw += dy * self.lookspeed;

            self.pitch = self.pitch.clamp(-f32::consts::PI / 2.0 * 0.99, f32::consts::PI / 2.0 * 0.99);
            self.yaw %= f32::consts::TAU;

            self.fvec = glam::Mat3::from_rotation_y(self.yaw)
                * glam::Mat3::from_rotation_x(self.pitch)
                * glam::Vec3::Z;
            self.rvec = self.fvec.cross(self.wupvec);
            self.uvec = self.rvec.cross(self.fvec);

            self.fvec = self.fvec.normalize();
            self.rvec = self.rvec.normalize();
            self.uvec = self.uvec.normalize();
        }

        // TODO: this doesn't work at all, need to fix this
        pub fn update_translation(&mut self, dx: f32, dy: f32, dz: f32) {
            self.pos += self.fvec * dz;
            self.pos += self.rvec * dx;
            self.pos += self.uvec * dy;
        }

        pub fn render<F>(&self, width: usize, height: usize, mut pixel_emitter: F)
        where
            F: FnMut(usize, usize, ray::Ray),
        {
            log::info!("Starting rendering on {}x{}", width, height);
            let basis = CameraRays::new(self, width, height);
            for py in 0..height {
                for px in 0..width {
                    pixel_emitter(
                        px,
                        py,
                        ray::Ray {
                            origin: self.pos,
                            direction: basis.point_to_pixel(px, py),
                            tspan: ray::RayInterval { low: ray::REAL_INTERVAL.low, high: self.renderdist },
                        },
                    );
                }
            }
            log::info!("Camera render finished");
        }
    }

    impl fmt::Display for Camera {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            writeln!(fmt, "P: {:.2}", self.pos)?;
            writeln!(fmt, "F: {:.2}", self.fvec)?;
            writeln!(fmt, "R: {:.2}", self.rvec)?;
            writeln!(fmt, "U: {:.2}", self.uvec)?;
            Ok(())
        }
    }
}
