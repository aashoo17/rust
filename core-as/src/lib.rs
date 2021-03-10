#![feature(const_raw_ptr_comparison)]
#![feature(slice_ptr_len)]
#![feature(slice_ptr_get)]
#![feature(ptr_internals)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(nonzero_leading_trailing_zeros)]
#![feature(maybe_uninit_extra)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

mod ptr;
mod num;
mod mem;
mod result;
mod option;
mod ops;
mod iter;

