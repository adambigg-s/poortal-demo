use crate::{mem::buffer, render::ray};

#[derive(Default, Debug, Clone, Copy)]
pub enum Species {
    #[default]
    None,
    Wall,
    Portal,
}

#[derive(bon::Builder, Default, Debug)]
pub struct Voxel {
    pub color: u32,
    pub species: Species,
}

#[derive(Default, Debug)]
pub struct Voxels {
    backing: buffer::Buffer<Voxel, 3>,
}

impl Voxels {}

impl ray::CastRay for Voxels {
    fn cast(&self, ray: ray::Ray) -> Option<ray::RayHit> {
        todo!()
    }
}
