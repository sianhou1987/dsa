use core::mem;
use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};
use std::marker::PhantomData;

struct Unique<T> {
    ptr: *const T,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Unique<T> {}
unsafe impl<T: Sync> Sync for Unique<T> {}

impl<T> Unique<T> {
    pub fn new(ptr: *mut T) -> Self {
        Unique {
            ptr: ptr,
            _marker: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr as *mut T
    }

    pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
        // SAFETY: the caller must guarantee that `ptr` is non-null.
        unsafe {
            Unique {
                ptr: ptr as _,
                _marker: PhantomData,
            }
        }
    }
}
enum AllocInit {
    /// The contents of the new memory are uninitialized.
    Uninitialized,
    /// The new memory is guaranteed to be zeroed.
    Zeroed,
}

fn capacity_overflow() -> ! {
    panic!("capacity overflow");
}

pub struct RawVec<T> {
    ptr: Unique<T>,
    cap: usize,
}

impl<T> RawVec<T> {
    /// Like `with_capacity`, but parameterized over the choice of
    /// allocator for the returned `RawVec`.
    #[inline]
    pub fn with_capacity_in(capacity: usize) -> Self {
        Self::allocate_in(capacity, AllocInit::Uninitialized)
    }

    /// Like `with_capacity_zeroed`, but parameterized over the choice
    /// of allocator for the returned `RawVec`.
    #[inline]
    pub fn with_capacity_zeroed_in(capacity: usize) -> Self {
        Self::allocate_in(capacity, AllocInit::Zeroed)
    }

    fn allocate_in(capacity: usize, init: AllocInit) -> Self {
        unsafe {
            if mem::size_of::<T>() == 0 {
                Self {
                    ptr: Unique::new_unchecked(mem::align_of::<T>() as *mut T),
                    cap: 0,
                }
            } else {
                let layout = match Layout::array::<T>(capacity) {
                    Ok(layout) => layout,
                    Err(_) => panic!("capacity overflow"),
                };
                let ptr = match init {
                    AllocInit::Uninitialized => alloc(layout),
                    AllocInit::Zeroed => alloc_zeroed(layout),
                };

                Self {
                    ptr: unsafe { Unique::new_unchecked(ptr as *mut T) },
                    cap: capacity,
                }
            }
        }
    }
}

impl<T> RawVec<T> {
    pub fn ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}
