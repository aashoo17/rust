// //what is NonNull and its usage
//
// //how NonNull<T> is covariant over T is it because it uses only one field which has *const T and we know that *const T is
// //covariant over T as this gives read only access to T
//
// //so NonNull is created same as Unique<T> use a *mut T to create NonNull and cast it to *const T
// //over this *const T NonNull will be wrapper
//
// use core::ptr::NonNull;
//
// pub fn create_nonnull() {
//     let a = &mut 10 as *mut i32;
//     //use only new which is safe
//     let mut b = NonNull::new(a).unwrap();
//     let c = unsafe { NonNull::new_unchecked(a) };
//
//     //get pointers
//     //TODO: why variable b which is NonNull is not getting consumed when we see all these 3 functions below take self as input
//     let d = b.as_ptr();
//     let e = unsafe { b.as_ref() };
//     let f = unsafe { b.as_mut() };
//
//     //cast
//     let g = b.cast::<u32>();
// }
//
// //NonNull<[T]> = how NonNull over arrays conceptually works as it has pointer only but no length
// //is it like c we have to provide length from outside and has to be careful that we are not going out of bounds
//
// pub fn nonnull_array() {
//     //so we should get a ptr to array in advance so that when creating NonNull<T>
//     let a = &mut [10, 20, 30, 40, 50] as *mut [i32];
//     let b = a.as_mut_ptr();
//     let c = NonNull::new(b).unwrap();
//     //create
//     let d = NonNull::slice_from_raw_parts(c, 5);
//     let e = d.len();
//     let f = d.as_non_null_ptr();
//     let g = d.as_mut_ptr();
//     //slice
//     let h = unsafe { d.get_unchecked_mut(1..4) };
//     //TODO: traits
// }
