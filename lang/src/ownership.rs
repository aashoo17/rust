/*
Ownership:
all the memory allocated in rust will have always one and only owner
and when owner goes out of scope typically after blocks {} memory will be cleaned up
 */
#[test]
fn ownership() {
    //memory is allocated on stack here for i32 variable
    let a = 10;
    //variable a goes out of scope here and memory 10 which was allocated gets cleaned up
}

/*
move:
so as we already know now that we can have only one owner to a memory
so operations which takes ownership like assignments, passing to function arg etc
will move the ownership from one variable to another
old variable will be Initialised now and if used rust wil throw error - value is moved
 */

#[test]
fn moves_ownership() {
    let a = String::from("Hello");
    //b takes ownership of memory pointed out by a now
    let b = a;

    //using a here will throw error
    print!("{}", b);
}
/*
Copy:
types which implement Copy trait does not flow move rule and instead of moving the memory
they will create a copy of themselves and pass
 */

#[test]
fn copy_type() {
    //i32 a Copy type variable
    let a = 10;
    //b here gets a copy of memory 10 pointed by a
    let b = a;
    //so a can be used here as it does not follow move semantics
    print!("{} {} ", a, b);
}

//TODO: show all types of operation which can move a value
