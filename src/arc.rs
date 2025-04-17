use core::{
    alloc::Layout,
    ptr::NonNull,
    sync::atomic::{
        AtomicUsize,
        Ordering::{Acquire, Relaxed, Release},
        fence,
    },
};

use alloc::alloc::{Allocator, Global, handle_alloc_error};

use crate::Resource;

const MAX_REFCOUNT: usize = (isize::MAX) as usize;
const INTERNAL_OVERFLOW_ERROR: &str = "ResArc counter overflow";

#[repr(C)]
struct ResInner {
    res: &'static Resource,
    strong: AtomicUsize,
}

fn layout(body: Layout) -> (Layout, usize) {
    Layout::new::<ResInner>()
        .extend(body)
        .unwrap_or_else(|_| handle_alloc_error(body))
}

impl ResInner {
    #[inline]
    fn body(&self) -> NonNull<()> {
        let (_, offset) = layout(self.res.layout);
        unsafe { NonNull::from_ref(self).cast::<()>().add(offset) }
    }
}

impl Drop for ResInner {
    fn drop(&mut self) {
        (self.res.drop)(self.body());
    }
}

pub(crate) struct ResArc<A: Allocator = Global> {
    ptr: NonNull<ResInner>,
    alloc: A,
}

unsafe impl<A: Allocator + Send> Send for ResArc<A> {}
unsafe impl<A: Allocator + Sync> Sync for ResArc<A> {}

impl<A: Allocator> ResArc<A> {
    fn new_in(res: &'static Resource, alloc: A) -> Self {
        let (layout, offset) = layout(res.layout);
        let ptr = alloc
            .allocate(layout)
            .unwrap_or_else(|_| handle_alloc_error(layout))
            .cast();

        unsafe {
            ptr.write(ResInner {
                strong: AtomicUsize::new(1),
                res,
            });
            (res.init)(ptr.cast().add(offset));
        }

        Self { ptr, alloc }
    }

    #[inline]
    fn header(&self) -> &ResInner {
        unsafe { self.ptr.as_ref() }
    }

    #[inline]
    fn body(&self) -> NonNull<()> {
        self.header().body()
    }
}

impl<A: Allocator + Clone> Clone for ResArc<A> {
    fn clone(&self) -> Self {
        let old_size = self.header().strong.fetch_add(1, Relaxed);
        assert!(old_size <= MAX_REFCOUNT, "{}", INTERNAL_OVERFLOW_ERROR);

        Self {
            ptr: self.ptr,
            alloc: self.alloc.clone(),
        }
    }
}

impl<A: Allocator> Drop for ResArc<A> {
    fn drop(&mut self) {
        if self.header().strong.fetch_sub(1, Release) != 1 {
            return;
        }

        fence(Acquire);

        let res = self.header().res;
        let (layout, offset) = layout(res.layout);

        unsafe {
            (res.drop)(self.ptr.cast().add(offset));
            self.alloc.deallocate(self.ptr.cast(), layout);
        }
    }
}
