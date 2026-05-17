pub struct PtrMut<T> {
    ptr: *mut T,
}

impl<T> PtrMut<T> {
    pub fn new(ptr: *mut T) -> Self {
        PtrMut { ptr }
    }

    pub fn as_raw(&self) -> *mut T {
        self.ptr
    }

    pub fn deref(&mut self) -> &mut T {
        unsafe { &mut (*self.ptr) }
    }
}

impl<T> Copy for PtrMut<T> {}

impl<T> Clone for PtrMut<T> {
    fn clone(&self) -> Self {
        *self
    }
}

unsafe impl<T> Send for PtrMut<T> {}

unsafe impl<T> Sync for PtrMut<T> {}
