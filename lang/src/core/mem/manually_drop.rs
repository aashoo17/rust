/*
Manually drop:
so this works like a type in c
you create it
do something with it
clean up explicitly - no automatic drop call/destructor run for you
 */

use core::mem::ManuallyDrop;

#[test]
fn manually_drop_memory() {
    //get the memory which will be manually managed
    //here 10 is an integer type which will be cleaned up manually
    let mut a = ManuallyDrop::new(10);
    //use the underlying value - since both Deref and DerefMut trait are implemented we can use * operator
    let b = *a;
    //change the value
    *a = 20;
    //drop it now
    //TODO: why drop is unsafe and why does it takes mutable reference to drop the memory
    unsafe {
        ManuallyDrop::drop(&mut a);
    }
}
