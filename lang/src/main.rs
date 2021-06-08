#![feature(ptr_internals)]
#![feature(maybe_uninit_extra)]
#![feature(const_raw_ptr_comparison)]
#![feature(slice_ptr_get)]
#![feature(slice_ptr_len)]
#![feature(nonnull_slice_from_raw_parts)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod closure;
mod collections;
mod concurrency;
mod core;
mod crates_modules;
mod enums;
mod error_handling;
mod expression;
mod io;
mod iterators;
mod lifetimes;
mod macros;
mod net;
mod operator_overloading;
mod ownership;
mod references;
mod structs;
mod tokio;
mod traits_generics;
mod types;
mod utility_traits;


fn main() {
    println!("Hello, world!");
}
