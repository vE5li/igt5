pub use std::alloc::{ AllocRef, Layout, Global, handle_alloc_error };
pub use std::ptr::{ NonNull, read, write };
pub use std::mem;

macro_rules! allocate {
    ($type:ty) => ({
        unsafe {
            let size = mem::size_of::<$type>();
            let align = mem::align_of::<$type>();
            let layout = Layout::from_size_align(size, align).unwrap();
            let mut pointer: NonNull<$type> = match Global.alloc(layout) {
                Ok(pointer) => pointer.cast::<$type>(),
                Err(_) => handle_alloc_error(layout),
            };
            pointer
        }
    });
}

macro_rules! deallocate {
    ($pointer:expr, $type:ty) => ({
        unsafe {
            let size = mem::size_of::<$type>();
            let align = mem::align_of::<$type>();
            let layout = Layout::from_size_align(size, align).unwrap();
            Global.dealloc($pointer.cast(), layout);
        }
    });
}
