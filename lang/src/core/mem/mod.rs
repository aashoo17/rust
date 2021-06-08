mod manually_drop;
mod maybe_uninit;

use core::mem;

//mem forget
#[test]
//so this will take ownership and destructor of type will not run
//so how memory is cleaned up
//may be we can rely on fact that stack values gets cleaned up when function exits anyway
//but don't use it for any heap resources, file handles etc which call destructor
//to explicitly clean big things whose memory leak will be huge
fn forget() {
    //not tried on copy type as moving ownership does not mean anything for them
    //TODO: but since we know that string keeps thing on heap so how can we ever clean it up
    let a = String::from("Hello World");
    mem::forget(a);
    //though use is not allowed in this print but memory still not cleared
    // println!("{}",a);
}

//get the size of types
#[test]
fn size_of() {
    //get int size
    assert_eq!(4, mem::size_of::<i32>());
    //TODO: get some other types sizes also
    //get float size

    //get array size

    //get reference & pointer size

    //get some struct size
}

//alignment of types
#[test]
fn align_of() {
    let a = mem::align_of::<i32>();
    println!("{}", a);
}

#[test]
fn swap() {
    let mut a = String::from("Hello");
    let mut b = String::from("World");

    //swap each others value without consuming either one
    mem::swap(&mut a, &mut b);
    println!("{}, {}", a, b);
}

#[test]
fn replace() {
    let a = String::from("Hello");
    let mut b = String::from("World");

    //replace b with value of a - a is consumed and can not be used further unless Copy type is used
    mem::replace(&mut b, a);
    println!("{}", b);
}

//drop
//run the types destructor in advance rather than waiting for variable to go out of scope and which will call destructor
//scope means generally {} block
fn drop() {
    let a = 10;
    //run a destructor before scope is reached
    mem::drop(a);
}
