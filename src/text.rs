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
