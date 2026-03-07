#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod closure;
mod conditional;
mod core;
mod crates_modules;
mod enums;
mod error_handling;
mod expression;
mod function;
mod io;
mod iterators;
mod lifetimes;
mod loops;
mod macros;
mod operator_overloading;
mod ownership;
mod references;
mod std;
mod structs;
mod tokio;
mod traits_generics;
mod types;
mod utility_traits;

fn main() {
    closure::run_all();
}
