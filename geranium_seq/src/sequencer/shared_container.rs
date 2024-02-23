use core::cell::UnsafeCell;

pub struct SharedContainer <T> (pub UnsafeCell<T>);

impl<T> SharedContainer <T> {
    pub fn borrow_mut(&self) -> &'static mut T {
        unsafe {
            return &mut *self.0.get();
        }
    } 
}
unsafe impl<T> Sync for SharedContainer<T> {}