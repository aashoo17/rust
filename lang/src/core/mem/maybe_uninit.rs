/*
this is used when we require variable which is not initialized
in normal circumstances rust compiler does not allow a variable to be un-initialized

MaybeUninit is a union which has 2 fields
1. unit tuple - () => this is kept in memory when not initialized
2. ManuallyDrop<T> => this will be kept when initialized

calling new() will initialize it and create ManuallyDrop<T> in union

calling uninit() will not initialize and will keep unit tuple in union memory

zeroed() - this will get the raw pointer to memory and make all bits as 0

write() - this is just call to new() and since now new() is called union is initialized and ManuallyDrop<T>
is lying in memory we can get the reference to this memory safely and hence write() returns a &mut T reference
otherwise if not initialized also we can use as_mut_ptr() to get writable raw pointer and write bytes to that location

as_ptr() & as_mut_ptr() can get you raw pointers to unions allocated memory location
once we get these we can do all *const T and *mut T stuff

assume_init() - this should be used when we want underlying type T which now be dropped with scope rule as usual
this internally calls ManuallyDrop::into_inner()

TODO: dropping union: if we don't call assume_init() and corresponding fn then how union will get dropped

 */

use core::mem::{self, MaybeUninit};

#[test]
fn maybe_uninit_variable() {
    //so this one initialized the variable
    let a = MaybeUninit::new(10);

    //getting uninitialized one
    let mut b = MaybeUninit::<i32>::uninit();

    //get zeroed value
    let mut c = MaybeUninit::<i32>::zeroed();

    //write new value (change underlying value)
    c.write(10);

    //drop the value if we have ownership of variable
    unsafe {
        c.assume_init();
    };

    //drop if ownership is not available - take mutably
    unsafe {
        b.assume_init_drop();
    }

    //TODO: creating arrays which are not initialized is unstable and may add in future to this exercise
}
