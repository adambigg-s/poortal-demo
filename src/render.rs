use crate::mem::buffer;

pub trait Raster {
    type Item;

    fn size(&self) -> [usize; 2];

    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn get(&mut self, x: usize, y: usize) -> &mut Self::Item;

    fn peek(&self, x: usize, y: usize) -> &Self::Item;
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

    fn peek(&self, x: usize, y: usize) -> &Self::Item {
        RenderTarget::get(self, [x, y])
    }
}

unsafe impl<T> Send for RenderTarget<T> {}

unsafe impl<T> Sync for RenderTarget<T> {}

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

            Self { start, drdx, dudy }
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
            self.pos += self.fvec * dz * self.movespeed;
            self.pos += self.rvec * dx * self.movespeed;
            self.pos += self.uvec * dy * self.movespeed;
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
