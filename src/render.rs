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

impl<T> Default for RenderTarget<T> {
    fn default() -> Self {
        Self::new([0, 0])
    }
}

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

    use crate::Raster;

    const FONT_SIZE: usize = 256;

    static DEFAULT_FONT: Font = Font::from_bytes(include_bytes!("../res/font/FONT_8x8.bin"));

    type BinRune = u64;

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

    #[derive(Clone, Copy)]
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
            R: Raster<Item = T>,
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
            R: Raster<Item = T>,
        {
            text.chars().for_each(|chr| {
                self.write_char(raster, chr);
            });
        }

        pub fn write_str_at<R>(&mut self, raster: &mut R, text: &str, col: usize, row: usize)
        where
            R: Raster<Item = T>,
        {
            self.set_pos(col, row);
            self.write_str(raster, text);
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
    }
}

pub mod camera {
    #[derive(Default, Debug)]
    pub struct CameraRays {
        drdx: glam::Vec3,
        dudy: glam::Vec3,
        hw: f32,
        hh: f32,
    }

    #[derive(Default, Debug)]
    pub struct Camera {
        pub fvec: glam::Vec3,
        pub rvec: glam::Vec3,
        pub uvec: glam::Vec3,
        pub wupvec: glam::Vec3,

        pub yaw: f32,
        pub pitch: f32,

        pub lookspeed: f32,
        pub movespeed: f32,

        pub fov: f32,
        pub ar: f32,
        pub renderdist: f32,
    }
}
