#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod closure;
mod core;
mod crates_modules;
mod enums;
mod error_handling;
mod expression;
mod io;
mod iterators;
mod lifetimes;
mod macros;
mod operator_overloading;
mod ownership;
mod references;
mod structs;
mod tokio;
mod traits_generics;
mod types;
mod utility_traits;
mod std;
mod loops;
mod conditional;
mod function;


fn main() {
    references::slices();
}
