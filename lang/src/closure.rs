//! # Closures in Rust
//!
//! Closures are anonymous functions that can capture values from their environment.
//!
//! ## Closure Traits
//! 1. `Fn`: Captures by immutable reference (`&T`).
//! 2. `FnMut`: Captures by mutable reference (`&mut T`).
//! 3. `FnOnce`: Captures by value (`T`).

/// Demonstrates an `Fn` closure (immutable borrow).
pub fn fn_closure() {
    let text: String = String::from("Hello");
    // The closure `print_text` implicitly borrows `text` immutably
    // because `println!` only requires read access to it.
    let print_text = || println!("Fn: {text}");
    
    print_text();
    print_text(); // Can be called multiple times
}

/// Demonstrates an `FnMut` closure (mutable borrow).
pub fn fnmut_closure() {
    let mut count = 0;
    // The closure mutates `count`, so it captures `count` by mutable reference.
    // The closure itself is declared `mut` because its internal state changes upon calling.
    let mut increment = || {
        count += 1;
        println!("FnMut: {count}");
    };
    
    increment();
    increment(); // Can mutate state
}

/// Demonstrates an `FnOnce` closure (takes ownership).
pub fn fnonce_closure() {
    let text: String = String::from("Consume me");
    // The closure moves `text` into `_moved`, consuming the captured variable.
    // As a result, this closure can only be executed once.
    let consume = || {
        let _moved = text; // Ownership transferred
        println!("FnOnce: variable consumed - {_moved}");
    };
    
    consume();
    // consume(); // Error: moved value
}

/// Demonstrates the `move` keyword (forces ownership transfer).
pub fn move_closure() {
    let data = vec![1, 2, 3];
    // The `move` keyword forces the closure to take ownership of `data`,
    // even though `println!` would typically only require an immutable reference.
    let force_move = move || println!("Moved data: {data:?}");
    
    force_move();
    // println!("{data:?}"); // Error: moved value
}

/// Demonstrates passing a closure as an argument.
pub fn closure_as_argument() {
    // A function that takes a generic closure `F` which implements the `Fn` trait.
    // The trait bound `Fn(i32) -> i32` enforces the expected parameter and return types.
    fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(x)
    }
    
    let double = |x| x * 2;
    println!("As argument (5 * 2): {}", apply(double, 5));
}

/// Demonstrates returning a closure from a function.
pub fn return_closure() {
    // A function that returns an opaque type implementing the `Fn` trait.
    // We must use `move` so the returned closure takes ownership of the argument `x`,
    // otherwise the closure would try to borrow a dropped local variable.
    fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y
    }
    
    let add_five = make_adder(5);
    println!("Returned closure (10 + 5): {}", add_five(10));
}

/// Demonstrates closures with standard library methods.
pub fn standard_library() {
    let numbers = vec![1, 2, 3];
    
    // .map() takes an `FnMut` closure to transform iterated elements. 
    // `numbers.iter()` immutably borrows the vector and yields references (`&i32`), 
    // which the closure then dereferences and squares.
    let _squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    
    // .filter() takes an `FnMut` closure that returns a boolean.
    // `into_iter()` takes ownership of `numbers`, but `filter` itself passes items 
    // to the closure by reference (`&i32`) so they aren't consumed during the check.
    let _evens: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("Stdlib iterators: closures applied to vec![1, 2, 3]");
}

/// Demonstrates closures in struct fields using dynamic dispatch.
pub fn closure_in_struct() {
    // When storing closures in structs, the exact type is conceptually known only at compile-time.
    // By using `Box<dyn Fn()>` we can lean on dynamic dispatch to store any matching closure.
    struct Processor {
        callback: Box<dyn Fn()>,
    }
    
    let p = Processor {
        callback: Box::new(|| println!("Action executed!")),
    };
    
    (p.callback)();
}

/// Wrapper to call all closure demonstration functions.
pub fn run_all() {
    println!("--- Fn Closure ---");
    fn_closure();
    
    println!("\n--- FnMut Closure ---");
    fnmut_closure();
    
    println!("\n--- FnOnce Closure ---");
    fnonce_closure();
    
    println!("\n--- Move Keyword ---");
    move_closure();
    
    println!("\n--- Closure as Argument ---");
    closure_as_argument();
    
    println!("\n--- Returning Closure ---");
    return_closure();
    
    println!("\n--- Standard Library closures ---");
    standard_library();
    
    println!("\n--- Closure in Struct ---");
    closure_in_struct();
}
