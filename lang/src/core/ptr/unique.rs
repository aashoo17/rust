/*
explained in mod.rs how we create safe abstraction over raw pointers using structs

TODO: why unique as well as NonNull are wrapper around *const T but said to be working as *mut T
why not wrapper over *mut T directly instead of *const T

it seems idea is we create Unique<T> using any *mut T and then cast this ptr to *const T
so that in normal cases if anybody is using it will be read only ptr
but we can be flexible when required data to change and cast *const T again to *mut T
similarly it is done in NonNull<T> also
But if ptr field of struct is private why not use *mut T directly why this casting from *const T to *mut T and vice versa.

TODO: why phantomData used in Unique<T>
TODO: subtyping and covariance explanation - i know these work on lifetime in rust but how on type T may be using trait as T: trait
TODO: why T: ?Sized i.e. why T is unsized in Unique<T>

subtyping and variance

TODO : explain any use cases of Unique<T>
 */

//create unique

use core::ptr::Unique;

#[test]
fn create() {
    let a = &mut 10 as *mut i32;
    //this version of new() is safe to use and should be used only
    let b = Unique::new(a).unwrap();
    //this will not be used in normal code usually
    let c = unsafe { Unique::new_unchecked(a) };

    //we could use coercion to get *mut T directly but type has to be hand written
    let d: *mut i32 = &mut 10;

    //TODO: what does dangling() do
    //so this just gives an aligned memory location without doing actual allocation, so memory could be invalid shared by many etc..
    //so this will be used for lazy allocation say
    //Vec::new() = does not have any element just keep some aligned address and when element is pushed do allocation and point there
    let e = Unique::<i32>::dangling();
    println!("{:?}", e);
}

//get underlying pointers as *mut T and as ref &T and &mut T
#[test]
fn get_underlying_ptr() {
    let a = &mut 10 as *mut i32;
    let mut b = Unique::new(a).unwrap();

    //TODO: is b is consumed being Unique<T> type - doesn't look like that why these types are not following usual rules
    //i thought raw pointers were exception but abstractions over them will follow usual rules
    //it may be due to Copy trait implemention also look for it

    //TODO: interesting point is that Unique is wrapper over *const T but we are getting is as *mut T, &T, &mut T
    //this means we were able to do conversion from *const T to => *mut T, &T and &mut T
    let c = b.as_ptr();

    //TODO: why these two functions are unsafe ?
    //TODO: what about lifetime when we get references here
    //the lifetime is bound to self i.e. Unique<T> if self is gone these references will not be valid
    //T means data underlying the raw pointer used
    let d = unsafe { b.as_ref() };
    let e = unsafe { b.as_mut() };

    //TODO: print here will throw error as when we get variable e we are taking it as mutable and now later in the pipe we can borrow as immutable after that
    //until mutable borrow is dropped
    //now with &T and &mut T in picture references/borrow rule is followed of rust that we can not have both mutable and immutable references
    //but i do learn that we can have &T before any &mut T uses in same scope
    //and this will not violate the rule that we can not have both mutable and immutable references
    //println!("{:?},{:?},{:?}",b,d,e);     //this print does not work as b is borrowed immutably in print macro
    //TODO: in single thread why can't we have mutable and immutable variable in same scope
}
