use crate::{mem::buffer, ray};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Species {
    #[default]
    None,
    Wall,
    Portal,
    Light,
}

#[derive(bon::Builder, Default, Debug, Clone, Copy)]
pub struct Voxel {
    pub color: u32,
    pub species: Species,
}

#[derive(Default, Debug)]
pub struct Voxels {
    pub backing: buffer::Buffer<Voxel, 3>,
}

impl Voxels {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { backing: buffer::Buffer::new([x, y, z]) }
    }

    pub fn clear(&mut self) {
        self.backing.fill(Voxel::default());
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, color: u32) {
        *self.backing.get_mut([x, y, z]) = Voxel { color, species: Species::Wall }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Voxel {
        *self.backing.get([x, y, z])
    }
}

impl ray::CastRay for Voxels {
    fn cast(&self, ray: ray::Ray) -> Option<ray::RayHit> {
        let [sx, sy, sz] = self.backing.size().map(|val| val as i32);
        let [stepx, stepy, stepz] = ray.direction.to_array().map(|comp| if comp > 0.0 { 1 } else { -1 });
        let [dx, dy, dz] = ray
            .direction
            .to_array()
            .map(|comp| if comp.abs() < 1e-9 { f32::MAX } else { (1.0 / comp).abs() });
        let [mut ix, mut iy, mut iz] = ray.origin.to_array().map(|val| val.floor() as i32);

        let mut tmax_x = if ray.direction.x > 0.0 {
            (ix as f32 + 1.0 - ray.origin.x) / ray.direction.x
        }
        else {
            (ray.origin.x - ix as f32) / -ray.direction.x
        };
        let mut tmax_y = if ray.direction.y > 0.0 {
            (iy as f32 + 1.0 - ray.origin.y) / ray.direction.y
        }
        else {
            (ray.origin.y - iy as f32) / -ray.direction.y
        };
        let mut tmax_z = if ray.direction.z > 0.0 {
            (iz as f32 + 1.0 - ray.origin.z) / ray.direction.z
        }
        else {
            (ray.origin.z - iz as f32) / -ray.direction.z
        };

        for _ in 0..ray.tspan.high as i32 {
            if ix < 0 || iy < 0 || iz < 0 || ix >= sx || iy >= sy || iz >= sz {
                return None;
            }

            let voxel = self.get(ix as usize, iy as usize, iz as usize);
            if voxel.species != Species::None {
                let time = tmax_x.min(tmax_y).min(tmax_z);
                return Some(ray::RayHit { time, color: voxel.color });
            }

            if tmax_x < tmax_y && tmax_x < tmax_z {
                tmax_x += dx;
                ix += stepx;
            }
            else if tmax_y < tmax_z {
                tmax_y += dy;
                iy += stepy;
            }
            else {
                tmax_z += dz;
                iz += stepz;
            }
        }

        None
    }
}
