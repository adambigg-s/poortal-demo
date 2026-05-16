use std::{f32, fmt};

use crate::ray;

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
    #[builder(default = glam::Vec3::Y)]
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
    fn update_vectors(&mut self) {
        self.fvec = glam::Mat3::from_rotation_y(self.yaw)
            * glam::Mat3::from_rotation_x(self.pitch)
            * glam::Vec3::NEG_Z;
        self.rvec = self.fvec.cross(self.wupvec);
        self.uvec = self.rvec.cross(self.fvec);

        self.fvec = self.fvec.normalize();
        self.rvec = self.rvec.normalize();
        self.uvec = self.uvec.normalize();
    }

    pub fn update_rotation(&mut self, dp: f32, dy: f32) {
        self.pitch += dp;
        self.yaw += dy;

        self.pitch = self.pitch.clamp(-f32::consts::PI / 2.0 * 0.99, f32::consts::PI / 2.0 * 0.99);
        self.yaw %= f32::consts::TAU;

        self.update_vectors();
    }

    pub fn update_rotation_delta(&mut self, dp: f32, dy: f32, dt: f32) {
        self.pitch += dp * dt;
        self.yaw += dy * dt;

        self.pitch = self.pitch.clamp(-f32::consts::PI / 2.0 * 0.99, f32::consts::PI / 2.0 * 0.99);
        self.yaw %= f32::consts::TAU;

        self.update_vectors();
    }

    pub fn update_translation(&mut self, dx: f32, dy: f32, dz: f32) {
        self.pos += self.fvec * dz;
        self.pos += self.rvec * dx;
        self.pos += self.wupvec * dy;
    }

    pub fn update_translation_delta(&mut self, dx: f32, dy: f32, dz: f32, dt: f32) {
        let forward = glam::vec3(self.fvec.x, 0.0, self.fvec.z).normalize_or_zero();
        let right = glam::vec3(self.rvec.x, 0.0, self.rvec.z).normalize_or_zero();
        let movement = (right * dx + self.wupvec * dy + forward * dz).normalize_or_zero();
        self.pos += movement * self.movespeed * dt;
    }

    pub fn render<F>(&self, width: usize, height: usize, mut pixel_emitter: F)
    where
        F: FnMut(usize, usize, ray::Ray),
    {
        let basis = CameraRays::new(self, width, height);
        (0..height).for_each(|py| {
            (0..width).for_each(|px| {
                pixel_emitter(
                    px,
                    py,
                    ray::Ray {
                        origin: self.pos,
                        direction: basis.point_to_pixel(px, py),
                        tspan: ray::RayInterval { low: ray::REAL_INTERVAL.low, high: self.renderdist },
                    },
                );
            });
        });
    }

    pub fn render_par<F>(&self, width: usize, height: usize, mut pixel_emitter: F)
    where
        F: FnMut(usize, usize, ray::Ray),
    {
        let basis = CameraRays::new(self, width, height);
        _ = (width, height, &mut pixel_emitter, basis);
        todo!()
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
