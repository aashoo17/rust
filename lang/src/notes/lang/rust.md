## Rust

## data types
i8,i16,i32....u8,u16...f32..
char,boolean

arrays and tuples
array slice
can we create tuple slice?

String
difference with char array and String
slice type &str vs String type

## Conditionals
if else if else

## Loop
loop,while, for in

function
passing parameter
returning

## Ownership and Borrow
ownership of data
two types of data => value type , reference type
value type => implementing Copy trait
how does complex types like vector,structs etc have ownership
of all the data contained in it
can we not have any refences inside these types as 
they will not own that part of data
what is lifetime specifier

what happens when we assign value type to two different variables and how does it differ from
assignment of reference types
concept of passing ownership  
passing a variable to function and passing ownership
borrow and mutable borrow
can both borrow and mutable borroe exist in same scope
why only one mutable borrow can only exist
at any time
what about immutable borrow

when we borrow do we get pointer in hand??

```rust
    let x = vec![1,2,3];
    for i in &x{
        *i += 2;    //dereferencing here
        println!(i);
    }
```

## struct
implementing methods on struct
:: vs .

## Enum
writing an enum
define what type will be conatined by the enum
can they have method which will return a type after calculation
implementing method on enums

Result and Option enum
are there elements brought into the scope

pattern matching

## Module
creating module structure
```rust
mod mod1{
    pub mod mod2{
        fn call(){

        }
    }
}
fn main(){
    mod1::mod2::call();
}

```
writing module code in seperate file and create the tree in main
```rust
//there will be mod1.rs file or mod1 folder with mod.rs file
mod mod1;
//there will be mod2.rs file or mod1 folder with mod.rs file
mod mod2;
fn main(){
    mod1::mod2::call();
}

```
understand the tree created by module structure
using external module
what is the structure of external module

use keyword for creating an alias
use mod1::mod2::call;
so call is the alias
creating other alias names

## vectors
creating with vec! macro and Vec::new()
pushing and popping new elements
getting individual elements
iterating over individual elemnts

## Strings
creating with String::from()
getting individual elements why not possible in rust
iterating over individual elemnts

## Hashmaps
creation
inserting elements
removing

## error handling
panic vs result enum 
using match
unwrap and expect function
using ?

## Generic types
how to write

## Traits
writing a trait
implementing it
can trait itself have default implementaion and you can use it
or overwrite it
passing trait variable in functions

## closures

closures can use any variable inside function without passing them as argument 
as in we do in function

```rust
fn main(){
    let age = 30;
    // we are not passing age in closure still able to use
    // as we have the access to age inside
    let clos = || println!("{}",age);
    //calling the closure
    clos();
}
```
closures can capture the variable as immutable ref or mutable ref 
compiler will do this automatically it will first try with immutable ref 
then only goes to mutable if required

if we want to pass the ownership we have to say move before closure
then closure can take the ownership
if value/variable implements copy trait it will be just copied inside closre
but refernce type will be moves permannetly 
that means copy type after moving in closure will still be accessible after closure 
but not reference type
```rust
fn main(){
    let str = String::from("Hello World!");
    let clos = move || println!("{}",str);
    clos();
    //we cant use str here cause it had been moved
    println!("{}",str);
}
```

passing and returning closure from function will require three traits to tell their type
using three traits Fn, FnMut, FnOnce

Fn: the closure captures by reference (&T)
FnMut: the closure captures by mutable reference (&mut T)
FnOnce: the closure captures by value (T)

For instance, consider a parameter annotated as FnOnce. This specifies that the closure may capture by &T, &mut T, or T, but the compiler will ultimately choose based on how the captured variables are used in the closure.

If the parameter is annotated as Fn, then capturing variables by &mut T or T are not allowed.


## Rust basics
[Rust basics](https://medium.com/learning-rust/rust-basics-e73304ab35c7)


## packages/modules

[Learning modules](https://medium.com/learning-rust/rust-lets-get-it-started-bdd8de58178d)