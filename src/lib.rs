//! [ArceOS](https://github.com/arceos-org/arceos) namespaces module.
//!
//! Namespaces are used to control system resource sharing between threads. This
//! module provides a unified interface to access system resources in different
//! scenarios.
//!
//! For a unikernel, there is only one global namespace, so all threads share
//! the same system resources, such as virtual address space, working directory,
//! and file descriptors, etc.
//!
//! For a monolithic kernel, each process corresponds to a namespace, all
//! threads in the same process share the same system resources. Different
//! processes have different namespaces and isolated resources.
//!
//! For further container support, some global system resources can also be
//! grouped into a namespace.

#![no_std]
#![feature(allocator_api)]
#![feature(decl_macro)]
#![feature(non_null_from_ref)]

extern crate alloc;

mod arc;

mod def;
pub use def::{RESOURCES, ResWrapper, Resource, def_resource};

mod ns;
pub use ns::Namespace;

#[crate_interface::def_interface]
pub trait AxNsIf {}
