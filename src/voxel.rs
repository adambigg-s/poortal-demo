use crate::{aabb, mem::buffer, ray};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Species {
    #[default]
    None,
    Wall,
    Portal,
}

#[derive(bon::Builder, Default, Debug, Clone, Copy)]
pub struct Voxel {
    #[builder(default = 0xff00ffff)]
    pub color: u32,
    #[builder(default = Species::Wall)]
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

    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut Voxel {
        self.backing.get_mut([x, y, z])
    }
}

pub struct VoxelHit {
    pub species: Species,
    pub color: u32,
    pub normal: glam::Vec3,
}

impl ray::CastRay for Voxels {
    type Hit = VoxelHit;

    fn cast(&self, ray: ray::Ray) -> Option<Self::Hit> {
        let aabb = aabb::AaBb::from(self.backing.size().map(|ele| ele as f32));
        let aabb_hit = aabb.cast(ray)?;
        if aabb_hit.ti > ray.tspan.high {
            return None;
        }

        let (dir, start_pos) = (ray.direction, ray.at(aabb_hit.ti));
        let step = dir.signum().as_ivec3();
        let delta = glam::vec3(
            if dir.x != 0.0 { dir.x.recip().abs() } else { f32::INFINITY },
            if dir.y != 0.0 { dir.y.recip().abs() } else { f32::INFINITY },
            if dir.z != 0.0 { dir.z.recip().abs() } else { f32::INFINITY },
        );

        let mut time = aabb_hit.ti;
        let mut vox = start_pos.floor().as_ivec3().clamp(
            glam::IVec3::ZERO,
            glam::ivec3(
                self.backing.size()[0] as i32 - 1,
                self.backing.size()[1] as i32 - 1,
                self.backing.size()[2] as i32 - 1,
            ),
        );
        #[rustfmt::skip]
        let mut side_dist = glam::vec3(
            if dir.x > 0.0 { ((vox.x + 1) as f32 - start_pos.x) * delta.x } else { (start_pos.x - vox.x as f32) * delta.x },
            if dir.y > 0.0 { ((vox.y + 1) as f32 - start_pos.y) * delta.y } else { (start_pos.y - vox.y as f32) * delta.y },
            if dir.z > 0.0 { ((vox.z + 1) as f32 - start_pos.z) * delta.z } else { (start_pos.z - vox.z as f32) * delta.z },
        );
        loop {
            if time > ray.tspan.high
                || vox.x < 0
                || vox.y < 0
                || vox.z < 0
                || vox.x >= self.backing.size()[0] as i32
                || vox.y >= self.backing.size()[1] as i32
                || vox.z >= self.backing.size()[2] as i32
            {
                return None;
            }

            let this_vox = self.get(vox.x as usize, vox.y as usize, vox.z as usize);
            if this_vox.species != Species::None {
                return Some(VoxelHit {
                    species: this_vox.species,
                    color: this_vox.color,
                    normal: glam::Vec3::ZERO,
                });
            }

            if side_dist.x < side_dist.y {
                if side_dist.x < side_dist.z {
                    time = side_dist.x;
                    side_dist.x += delta.x;
                    vox.x += step.x;
                }
                else {
                    time = side_dist.z;
                    side_dist.z += delta.z;
                    vox.z += step.z;
                }
            }
            else {
                if side_dist.y < side_dist.z {
                    time = side_dist.y;
                    side_dist.y += delta.y;
                    vox.y += step.y;
                }
                else {
                    time = side_dist.z;
                    side_dist.z += delta.z;
                    vox.z += step.z;
                }
            }
        }
    }
}
