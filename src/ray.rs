use std::ops;

pub const EPS_RAY: f32 = 1e-6;
pub const RAY_OFFSET: f32 = 1e-3;
pub const REAL_INTERVAL: RayInterval = RayInterval { low: f32::EPSILON, high: f32::MAX };

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Axis {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl From<Axis> for usize {
    fn from(val: Axis) -> Self {
        match val {
            | Axis::PosX => 0,
            | Axis::NegX => 0,
            | Axis::PosY => 1,
            | Axis::NegY => 1,
            | Axis::PosZ => 2,
            | Axis::NegZ => 2,
        }
    }
}

impl ops::Not for Axis {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            | Axis::PosX => Axis::NegX,
            | Axis::NegX => Axis::PosX,
            | Axis::PosY => Axis::NegY,
            | Axis::NegY => Axis::PosY,
            | Axis::PosZ => Axis::NegZ,
            | Axis::NegZ => Axis::PosZ,
        }
    }
}

pub trait CastRay {
    type Hit;

    fn cast(&self, ray: Ray) -> Option<Self::Hit>;
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
