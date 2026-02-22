/**
 * # References and Borrowing
 *
 * Unlike owners, references do not drop memory when they go out of scope.
 * However, they must always point to valid memory (the owner must outlive the reference).
 *
 * ## Lifetimes
 * The compiler uses lifetimes to ensure references never outlive their owners.
 * While usually inferred, explicit lifetime annotations are sometimes required
 * when the compiler cannot determine the relationship between references.
 *
 * ## Borrowing Rules
 * At any given time, you can have either:
 * 1. Any number of immutable references (`&T`).
 * 2. Exactly one mutable reference (`&mut T`).
 * 3. Immutable and mutable references cannot coexist if their usages overlap.
 */
pub fn working_with_references() {
    // Note: This code compiles because of Non-Lexical Lifetimes (NLL).
    // The compiler recognizes that `b` is never used, so its borrow ends
    // immediately, allowing `c` (the mutable borrow) to start.
    // otherwise by seeing this it may feel both mutable and non-mutable borrow using b and c is allowed
    let mut message = String::from("Hello World");
    let b = &message;      // Immutable borrow starts - since it is not used later this works
    let c = &mut message;  // Mutable borrow starts and ends here (unused)
    println!("{:?}", c); // This line uses `c`, so its borrow extends to here.
}

/**
 * # Slices
 *
 * Slices are references to a contiguous sequence of elements in a collection rather than the whole collection.
 *
 * ## Types of Slices
 * - **Shared/Mutable References**: Simple types `T` use `&T` or `&mut T`.
 * - **Array/Vec Slices**: Represented as `&[T]`, providing a view into a block of memory.
 * - **String Slices**: Represented as `&str`. This is an immutable reference to a UTF-8 sequence.
 *
 * ## Memory Locations
 * A `&str` can point to:
 * 1. The binary (string literals).
 * 2. The stack (fixed-size arrays).
 * 3. The heap (from a `String`).
 */
pub fn slices(){
    //arrays and slices
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];
    let full_array_slice: &[i32] = &numbers[..];
    let partial_array_slice: &[i32] = &numbers[1..3];
    println!("{:?} {:?} {:?}", numbers, full_array_slice, partial_array_slice);

    // Slicing a String [..] is safe. However, slicing with indices like [0..3]
    // will panic at runtime if the index falls within a multi-byte UTF-8 character
    // (e.g., slicing in the middle of an emoji).
    let heap_string: String = String::from("Hello World");
    let full_string_slice: &str = &heap_string[..];
    let static_string: &str = "Hello World"; // This is a string literal, stored in the binary
    println!("{:?} {:?} {:?}", heap_string, static_string, heap_string);

    //byte array (also can be used as string on stack for ascii chars) and slice
    let byte_array: &[u8; 5] = b"hello"; // Byte array (string also) on the stack
    let byte_slice: &[u8] = &byte_array[..];
    println!("{:?} {:?}", byte_array, byte_slice)
}
