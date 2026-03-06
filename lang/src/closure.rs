// [understanding the closure concept in rust](https://internals.rust-lang.org/t/still-confused-by-move-vs-closures/1430)

/*
Fn, FnMut, FnOnce Traits:
These traits are automatically implemented based on how the closure *uses* the captured variables.
- Fn:     Captures/uses variables immutably (&T). Can be called multiple times.
- FnMut:  Captures/uses variables mutably (&mut T). Can be called multiple times.
- FnOnce: Captures/uses variables by value (T). Can be called at least once (consumes the closure).
*/

// Fn closure: Captures variables from the environment immutably.
// Can be called multiple times concurrently.
pub fn fn_closure() {
    let greeting = String::from("Hello World");

    // This closure captures `greeting` by reference (&T) because `println!` only needs a reference.
    // It implements `Fn`.
    let print_greeting = || {
        println!("Fn closure: {}", greeting);
    };

    // Since `print_greeting` is `Fn`, it can be called multiple times.
    print_greeting();
    print_greeting();

    // `greeting` is still valid here because it was only borrowed.
    println!("Main: {}", greeting);
}

// FnMut closure: Captures variables from the environment mutably.
// Can be called multiple times, changing the captured state.
pub fn fnmut_closure() {
    let mut data = String::from("Hello");

    // This closure captures `data` by mutable reference (&mut T) because we modify it.
    // It implements `FnMut`.
    let mut append_world = || {
        data.push_str(" World");
        println!("FnMut closure: {}", data);
    };

    // Since `append_world` is `FnMut`, it can be called multiple times.
    append_world();
    append_world();
}

// FnOnce closure: Consumes (moves) variables from the environment.
// Can be called ONLY once.
pub fn fnonce_closure() {
    let temporary_value = String::from("I will be consumed");

    // This closure captures `temporary_value` by value (T) because we force a move
    // (implicitly or explicitly) that consumes ownership.
    // It implements `FnOnce`.
    let consume_value = || {
        let _taken = temporary_value; // Ownership moved here
        println!("FnOnce closure: consumed variable");
    };

    consume_value();
    // consume_value(); // Error: use of moved value: `consume_value`
}

// Demonstrates the `move` keyword.
// `move` forces the closure to take ownership of values it uses,
// regardless of whether it actually *needs* ownership to function.
pub fn move_keyword_example() {
    let ownable_string = String::from("I am owned by the closure now");

    // The 'move' keyword forces capture-by-value.
    // Even though we only print (read) it, the closure now owns `ownable_string`.
    let owning_closure = move || {
        println!("{}", ownable_string);
    };

    owning_closure();
    // println!("{}", ownable_string); // Error: value borrowed here after move
}
