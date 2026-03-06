/*
 * # References and Borrowing
 *
 * References allow you to refer to a value without taking ownership of it.
 * Unlike owned values, the memory they point to isn't dropped when the reference goes out of scope.
 *
 * ## Borrowing Rules
 * Rust enforces two main rules for references (borrowing):
 * 1. At any given time, you can have either exactly one mutable reference OR any number of immutable references.
 * 2. References must always be valid (the data they point to must outlive the reference).
 *
 * ## Non-Lexical Lifetimes (NLL)
 * Modern Rust uses Non-Lexical Lifetimes. This means a reference's lifetime ends at its
 * last point of use, rather than at the end of the enclosing block's scope. This allows
 * for more flexible code where a mutable borrow can begin immediately after an immutable
 * borrow's final usage.
 */
pub fn working_with_references() {
    let mut message: String = String::from("Hello World");
    
    // Note: This compiles because of Non-Lexical Lifetimes (NLL).
    // The immutable borrow `message_ref` ends after its last use in the first println!.
    // This allows the mutable borrow `message_mut` to be created afterward safely.
    let message_ref: &str = &message;
    println!("Immutable borrow: {message_ref}");
    
    let message_mut: &mut String = &mut message;
    message_mut.push_str("!");
    println!("Mutable borrow: {message_mut}");
    
    // If we tried to use `message_ref` here, it would be a compiler error because 
    // `message_mut` (a mutable borrow) overlaps with `message_ref`'s scope.
    // println!("{message_ref}");
}

/*
 * # Slices
 *
 * Slices are references to a contiguous sequence of elements in a collection, 
 * rather than the whole collection. Since slices are references, they do not have ownership.
 *
 * ## Types of Slices
 * - **Shared/Mutable References**: Simple types `T` use `&T` or `&mut T`.
 * - **Array/Vec Slices**: Represented as `&[T]`, providing a view into a block of memory.
 * - **String Slices**: Represented as `&str`. This is an immutable reference to a UTF-8 sequence.
 *
 * ## Memory Locations
 * A slice can point to data anywhere in memory:
 * 1. The binary/executable (e.g., string literals `&str`).
 * 2. The stack (e.g., fixed-size arrays `&[T]`).
 * 3. The heap (e.g., from a `String` or `Vec<T>`).
 */
pub fn slices() {
    // 1. Arrays and Slices (Stack)
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];
    let full_slice: &[i32] = &numbers[..];
    let partial_slice: &[i32] = &numbers[1..3];
    
    println!("Full numerical slice: {full_slice:?}");
    println!("Partial numerical slice: {partial_slice:?}");

    // 2. String Slices (Heap vs Binary)
    // Note: &str is a slice regardless of whether it points to the Heap or Binary
    let heap_string: String = String::from("Hello World");
    let full_string_slice: &str = &heap_string[..]; // Points to the Heap
    let static_string: &str = "Hello World";        // Points to the Binary (String literal)
    
    println!("Heap string slice: {full_string_slice}");
    println!("Static string slice: {static_string}");

    // 3. Byte Arrays and Slices
    // b"hello" is a byte string literal of type &[u8; 5]
    let byte_array: &[u8; 5] = b"hello";
    let byte_slice: &[u8] = &byte_array[1..4]; // Slice of bytes: [101, 108, 108]

    // Printing bytes as characters requires a bit of help,
    // but for debugging, {:?} shows the ASCII decimal values.
    println!("Byte Array: {byte_array:?}, Byte Slice: {byte_slice:?}");
}
