use crate::mem::buffer;

pub trait Raster {
    type Item;

    fn size(&self) -> [usize; 2];

    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn get(&mut self, x: usize, y: usize) -> &mut Self::Item;

    fn peek(&mut self, x: usize, y: usize) -> &Self::Item;
}

pub type RenderTarget<T> = buffer::Buffer<T, 2>;

impl<T> Default for RenderTarget<T> {
    fn default() -> Self {
        Self::new([0, 0])
    }
}

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

    fn peek(&mut self, x: usize, y: usize) -> &Self::Item {
        RenderTarget::get(self, [x, y])
    }
}

unsafe impl<T> Send for RenderTarget<T> {}

unsafe impl<T> Sync for RenderTarget<T> {}
