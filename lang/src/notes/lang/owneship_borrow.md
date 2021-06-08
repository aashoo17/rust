## Ownership
only one variable will own a single piece of memory

```rust
fn main(){
    //variable a is owner of the String memory created somewhere
    let a = String::from("Hello World");
    //b and a both together can not be owner of same memory
    let b = a;
    //if we try to print a it will not be accessible any more
    //so new owner of this memory is b and a not accessible any more this is called moving/transferring the ownership
    println!("{}",a);
}
```
primitive types like i32 etc if we will check it will seem transfer didn't happen
```rust
fn main(){
    let a = 10;
    let b = a;
    //a is still accessible with value 10 unlike it happened with String
    //so if a type implements a trait called Copy we will actually create a new copy of memory when b = a happens
    //so both a and b got different memory to point to here
    //but as we said one memory one owner is still valid here
    println!("{}",a);
}
```

## Scope
scope is pretty much like this  
{    
    this is the scope of a variable
}  

so when owner goes beyond its scope not only it can no longer be accessed but associated memory is also cleaned up  

## Borrowing
1. immutable borrow 
2. mutable borrow  

immutable borrows are read only and can create any nos u want but 
mut borrow can be taken one in a scope and no immut 
borrow if there is mut one in same scope  
if the borrows goes out of scope nothings happen to underlying data  

apart from assignment function taking arguments also move the ownership

```rust
fn main(){
    let a = String::from("Hello World");
    //this assignment will move the ownership
    let b = a;
    //passing variable a to function here also moves the ownership 
    call(a);
}

fn call(x: String){

}
```

# Copy trait
so movement of ownership is not desired always so some types
implement copy trait which means these types will be copied 
rather passing ownership