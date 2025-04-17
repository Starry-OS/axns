use core::{alloc::Layout, ptr::NonNull};

use alloc::alloc::{Allocator, Global, handle_alloc_error};

use crate::{RESOURCES, arc::ResArc};

pub struct Namespace<A: Allocator = Global> {
    // Not using [ResArc<A>] to save a `usize` because we know the length
    ptr: NonNull<ResArc<A>>,
    alloc: A,
}

unsafe impl<A: Allocator + Send> Send for Namespace<A> {}
unsafe impl<A: Allocator + Sync> Sync for Namespace<A> {}

impl<A: Allocator> Namespace<A> {
    pub fn layout() -> Layout {
        Layout::array::<ResArc<A>>(RESOURCES.len()).unwrap()
    }

    pub fn new_in(alloc: A) -> Self {
        let layout = Self::layout();
        let ptr = alloc
            .allocate(layout)
            .unwrap_or_else(|_| handle_alloc_error(layout))
            .cast();

        let s = unsafe { core::slice::from_raw_parts_mut(ptr.as_ptr(), RESOURCES.len()) };

        Self { ptr, alloc }
    }
}
