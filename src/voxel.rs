use crate::{mem::buffer, ray};

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

pub struct VoxelHit {}

impl ray::CastRay for Voxels {
    type Hit = VoxelHit;

    fn cast(&self, ray: ray::Ray) -> Option<Self::Hit> {
        None
    }
}

