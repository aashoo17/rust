// #![feature(ptr_internals)]
// #![allow(dead_code)]
//
// /*
// explained in mod.rs how we create safe abstraction over raw pointers using structs
//
// Unique<T> and NonNull<T> use *const T to opt-into COVARIANCE.
// *mut T is invariant, meaning if you have Unique<Subtype>, you couldn't use it where Unique<Supertype> is expected.
// By using *const T internally, we tell the compiler that Unique<T> is covariant over T.
//
// PhantomData<T> is used for two reasons:
// 1. It tells the compiler that Unique<T> "owns" a T, which affects the drop checker (dropck).
// 2. It ensures that if T is Send/Sync, Unique<T> can correctly derive those traits (or not).
//
// T: ?Sized allows Unique to wrap "unsized" types like [u8] or dyn Trait (DSTs).
// Without ?Sized, Unique would only work with types whose size is known at compile time.
//
// Use cases: Unique<T> is the internal building block for Box<T>, Vec<T>, and String.
//
// subtyping and variance
//  */
//
// //create unique
//
// use std::ptr::Unique;
//
// #[test]
// fn create() {
//     let mut val = 10;
//     let a = &mut val as *mut i32;
//     //this version of new() is safe to use and should be used only
//     let b = Unique::new(a).unwrap();
//     //this will not be used in normal code usually
//     let c = unsafe { Unique::new_unchecked(a) };
//
//     //we could use coercion to get *mut T directly but type has to be hand written
//     let mut val2 = 10;
//     let d: *mut i32 = &mut val2;
//
//     // dangling() gives an aligned memory location without doing actual allocation,
//     // so memory could be invalid or shared by many.
//     //so this will be used for lazy allocation say
//     //Vec::new() = does not have any element just keep some aligned address and when element is pushed do allocation and point there
//     let e = Unique::<i32>::dangling();
//     println!("{:?}", e.as_ptr());
// }
//
// //get underlying pointers as *mut T and as ref &T and &mut T
// #[test]
// fn get_underlying_ptr() {
//     let mut val = 10;
//     let a = &mut val as *mut i32;
//     let mut b = Unique::new(a).unwrap();
//
//     // Unique<T> does NOT implement Copy (unlike NonNull). It represents ownership.
//     // However, as_ptr() takes &self, so it doesn't consume 'b'.
//     let c = b.as_ptr();
//
//     // These are unsafe because the compiler cannot guarantee the pointer is still valid,
//     // aligned, or that you aren't violating aliasing rules (multiple &mut).
//     unsafe {
//         let d = b.as_ref();
//         println!("Ref: {:?}", d);
//
//         // We must drop or stop using 'd' before creating 'e' to satisfy the borrow checker.
//         let e = b.as_mut();
//         *e += 1;
//         println!("Mut Ref: {:?}", e);
//     }
//
//     // In a single thread, we can't have & and &mut simultaneously because it prevents
//     // "iterator invalidation" and data races where one part of the code expects
//     // data to be constant while another changes it.
//     println!("Final pointer: {:?}", b.as_ptr());
// }
