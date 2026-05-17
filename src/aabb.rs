use std::cmp;

use crate::ray;

#[derive(bon::Builder, Debug, Clone, Copy)]
pub struct AaBb<T, const N: usize> {
    pub lo: [T; N],
    pub hi: [T; N],
}

impl<T, const N: usize> AaBb<T, N> {
    pub fn new(low: [T; N], high: [T; N]) -> Self {
        Self { lo: low, hi: high }
    }

    pub fn overlaps(&self, other: &Self) -> bool
    where
        T: cmp::PartialOrd,
    {
        (0..N).all(|dim| self.lo[dim] <= other.hi[dim] && self.hi[dim] >= other.lo[dim])
    }
}

impl<T, const N: usize> Default for AaBb<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self { lo: [T::default(); N], hi: [T::default(); N] }
    }
}

impl<T, const N: usize> From<[T; N]> for AaBb<T, N>
where
    T: Default + Copy,
{
    fn from(hi: [T; N]) -> Self {
        Self { lo: [T::default(); N], hi }
    }
}

#[derive(bon::Builder, Debug, Clone, Copy)]
pub struct AaBbHit {
    pub ti: f32,
    pub to: f32,
    pub axis: ray::Axis,
}

impl ray::CastRay for AaBb<f32, 3> {
    type Hit = AaBbHit;

    fn cast(&self, ray: ray::Ray) -> Option<Self::Hit> {
        let (orig, dir) = (ray.origin, ray.direction);

        let inv = glam::vec3(
            if dir.x != 0.0 { dir.x.recip() } else { f32::INFINITY },
            if dir.y != 0.0 { dir.y.recip() } else { f32::INFINITY },
            if dir.z != 0.0 { dir.z.recip() } else { f32::INFINITY },
        );

        let lo = glam::Vec3::from(self.lo);
        let hi = glam::Vec3::from(self.hi);

        let t1 = (lo - orig) * inv;
        let t2 = (hi - orig) * inv;

        let tmin = t1.min(t2);
        let tmax = t1.max(t2);

        let tentry = tmin.x.max(tmin.y).max(tmin.z);
        let texit = tmax.x.min(tmax.y).min(tmax.z);

        if texit < 0.0 || tentry > texit || tentry > ray.tspan.high {
            return None;
        }

        let mut axis = {
            if tmin.x > tmin.y {
                if tmin.x > tmin.z { ray::Axis::PosX } else { ray::Axis::PosZ }
            }
            else {
                if tmin.y > tmin.z { ray::Axis::PosY } else { ray::Axis::PosZ }
            }
        };
        if dir[axis.into()] > 0.0 {
            axis = !axis
        };

        Some(AaBbHit { ti: tentry.max(ray.tspan.low), to: texit, axis })
    }
}
