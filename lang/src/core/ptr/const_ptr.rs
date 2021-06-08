//using nightly version
//as some apis are unstable and can not be called from stable rust version

use core::ptr::null;
use core::ptr::null_mut;

//write all the known ways by which i can get the *const T
#[test]
fn create_const_ptr() {
    //from a variable
    let a = 10;
    let _b = &a as *const i32;

    //directly from literal
    let _c = &10 as *const i32;

    //what the f**k we can get coercion without explicitly saying: &T as *const T
    //may be by writing the type we are explicit
    //this seems to be best version - small and simple
    let d: *const i32 = &10;
    println!("{:?}", d);

    //TODO: may be get pointers from arrays/slices
}

//getting null
#[test]
fn get_null_value() {
    let a = null::<i32>();
    let b = null_mut::<i32>();
    println!("{:?},{:?}", a, b);
}

/*
now if we see method signatures over *const T and *mut T they take self as first parameter so
they should consume the variable but that does not happens
it could have used just associated methods and called using :: operator
because most of the time self is not used inside method for anything at all
TODO: explain it how this works
may be they are copy type and passed as copy but then again how we will modify them ??
that will happen only if return of function is assigned back to them. so ??
look your method and find out
 */

//checking null
#[test]
fn is_null() {
    let a = &20 as *const i32;
    //TODO: how to get the null pointer & check if is_null() is true on it
    //checking that pointer is not null
    //weird thing is that it will consume the pointer/varibale so what does i will do after checking about nullability
    //it seems we can still use the a below how a is still valid, is it because *const T is Copy type
    //TODO: find out how a is still accessible after consumed as self
    let _b = a.is_null();
    //see here now a is no longer available
    print!("{:?}", a);
}

//casting to another pointer type
//so casting is safe since no harm will be done as deref is unsafe anyway
#[test]
fn cast() {
    let a = &20 as *const i32;

    //cast it to say u32 pointer type
    //since its generic function without any arg specify the type using ::<T> syntax
    let _b = a.cast::<u32>();
}

//get as reference as &T
#[test]
fn as_ref() {
    let a = &20 as *const i32;
    //I have already explained it why getting references is unsafe in mod.rs file
    //TODO: find out lifetime which rust is giving them - or telling them till when they are valid
    if let Some(b) = unsafe { a.as_ref() } {
        print!("{}", b)
    }
}

/*
offset
so as we know from c that we can +, - any integer to get new pointers in both direction best used in array
same is offset in rust
wrapping offset
TODO: see if my idea of wrapping offset is correct
ptr + isize
to do this ptr is converted to isize first then added to isize type now if overflow happens during addition/substraction wrap
around arithmetic is used instead of overflowing
 */
#[test]
fn offset() {
    let a = &20 as *const i32;
    //TODO: why offset is unsafe and wrapping offset is safe
    //cause first i was thinking with offset i can reach any memory and so unsafe but same wrapping offset
    //can do only thing wrapping_offset can guarantee is pointer never overflows usize (8 bytes) capacity but wraps
    let b = unsafe { a.offset(20) };
    println!("{:?}", b);

    //wrapping offset
    //TODO: since if we wrap we will get any valid address location this is not unsafe function ??
    //but 8 bytes may even store address which will never fall in actual cpu so we still get an invalid address if not overflowing
    let c = a.wrapping_offset(20);

    println!("{:?}", c);

    //offset_from(): get the offset value between 2 pointers
    let d = unsafe { c.offset_from(a) };
    println!("{:?}", d);

    //add and sub based on offset idea
    let e = unsafe { a.add(20) };
    println!("{:?}", e);

    let f = unsafe { a.sub(20) };
    println!("{:?}", f);

    //similarly wrapping_add() and wrapping_sub() can be called
}

//checking equality
//TODO: since ==, <= etc are implemented on *const T using trait so why these function calls ?

//#![feature(const_raw_ptr_comparison)] feature is to be enabled to use this
#[test]
fn equality() {
    let a = &20 as *const i32;
    let b = &30 as *const i32;

    let c = a.guaranteed_eq(b);
    let d = a.guaranteed_ne(b);

    assert_ne!(c, true);
    assert_eq!(d, true);
}

//TODO: set_ptr_value() function understand then write usage of it

//read the underlying value
//TODO: understand the other read versions like read_volatile() & read_aligned() etc
//TODO: how this is different than deref and get the value
#[test]
fn read() {
    let a = &20 as *const i32;

    //so is it like deref the value and get underlying value *a
    let b = unsafe { a.read() };

    //let b = unsafe { *a }; => this could have also been used
    print!("{}", b);
}

//copy the underlying data at the pointer to other location
#[test]
fn copy() {
    let a = &20 as *const i32;
    let b = &mut 30 as *mut i32;
    //this copy will work even if memory is overlapping
    unsafe { a.copy_to(b, 1) };
    //TODO: just using read instead of deref *ptr as i still don't know the difference b/w them
    let c = unsafe { a.read() };
    println!("{}", c);

    //copying non overlapping memory
    //will this be faster than copy_to() version
    unsafe { a.copy_to_nonoverlapping(b, 1) };
    let d = unsafe { a.read() };
    println!("{}", d);
}

// Computes the offset that needs to be applied to the pointer in order to make it aligned to
// `align`.
#[test]
fn align_offset() {
    let a = &20 as *const i32;
    //align need to be any power of 2, 2^4 = 16 here
    let b = a.align_offset(16);
    println!("{}", b);
}

//traits like equality, comparison ==, <= etc on *const T
#[test]
fn traits() {
    //keep both pointer same to see if equality,comparison etc doesn't depend upon underlying value of pointer or not
    let a = &20 as *const i32;
    let b = &20 as *const i32;

    //so my understanding was equal should be when both address is same
    //and less or greater should be by comparison of address
    //but it doesn't look like this
    //TODO: it seems it is taking into consideration the underlying value for equality and comparison
    println!("{},{}", a == b, a < b);
}

//api over *const [T]

//how to create these
#[test]
fn const_array_ptr() {
    let a = &[10, 20, 30, 40, 50] as *const [i32];
    //same thing done in 2 lines
    let b = [10, 20, 30, 40, 50];
    let _c = &b as *const [i32];
    // let d: *const [i32] = &[10,20,30,40,50];     //i thought use the implicit coercion here also but doesn't seems to work
    println!("{:?}", a);
}

#[test]
fn len() {
    let a = &[10, 20, 30, 40, 50] as *const [i32];
    //once converted to pointer how could have it known the size now - so *const [T] may be keeping size after all
    let b = a.len();
    assert_eq!(b, 5);
}

#[test]
fn as_ptr() {
    let a = &[10, 20, 30, 40, 50] as *const [i32];
    //get the pointer to first element
    let b = a.as_ptr();

    //read the value and make sure it is 10 as we must have got ptr to first element
    let c = unsafe { b.read() };
    assert_eq!(c, 10);
}

//get the slice
#[test]
fn get_slice() {
    let a = &[10, 20, 30, 40, 50] as *const [i32];
    //get 1 element
    let b = unsafe { a.get_unchecked(1) };
    //slice
    let c = unsafe { a.get_unchecked(1..3) };
}
