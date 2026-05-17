use crate::mem::buffer;

unsafe impl<T, const N: usize> Send for buffer::Buffer<T, N> {}

unsafe impl<T, const N: usize> Sync for buffer::Buffer<T, N> {}

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
