use core::{alloc::Layout, marker::PhantomData, ops::Deref, ptr::NonNull};

pub struct Resource {
    pub layout: Layout,
    pub init: fn(NonNull<()>),
    pub drop: fn(NonNull<()>),
}

#[linkme::distributed_slice]
pub static RESOURCES: [Resource];

impl Resource {
    pub fn index(&'static self) -> usize {
        unsafe { (self as *const Resource).offset_from_unsigned(RESOURCES.as_ptr()) }
    }
}

#[derive(Clone, Copy)]
pub struct ResWrapper<T>(&'static Resource, PhantomData<T>);

impl<T> ResWrapper<T> {
    pub const fn new(res: &'static Resource) -> Self {
        Self(res, PhantomData)
    }
}

impl<T> Deref for ResWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

pub macro def_resource {
    ( $( $(#[$attr:meta])* $vis:vis static $name:ident: $ty:ty = $default:expr; )+ ) => {
        $(
            #[linkme::distributed_slice($crate::RESOURCES)]
            static RES: $crate::Resource = $crate::Resource {
                layout: core::alloc::Layout::new::<$ty>(),
                init: |ptr| {
                    let val = $default;
                    unsafe { ptr.cast().write(val) }
                },
                drop: |ptr| unsafe {
                    ptr.cast::<$ty>().drop_in_place();
                },
            };
            const _: () = assert!(RES.layout.size() != 0, "Resource has zero size");

            #[used]
            #[doc(hidden)]
            $(#[$attr])*
            $vis static $name: $crate::ResWrapper<$ty> = $crate::ResWrapper::new(&RES);
        )+
    }
}
