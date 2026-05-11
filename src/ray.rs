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
