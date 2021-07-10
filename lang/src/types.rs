#[test]
fn integers() {
    //putting i8, i16 etc at the end of literal so rust recognizes it to be i8,i16 otherwise it will thinks as i32
    //or we can explicitly write the type like these let a: i8 = 10;
    //signed integers
    let a = 10i8;
    let b = 10i16;
    let c = 10;
    let d = 10i64;

    //[how rust supports 128 bit integers](https://stackoverflow.com/questions/57340308/how-does-rusts-128-bit-integer-i128-work-on-a-64-bit-system)
    let e = 10i128;
    let f = 10isize;

    //unsigned integer types
    let g = 10u8;
    let h = 10u16;
    let i = 10u32;
    let j = 10u64;
    let k = 10u128;
    let l = 10usize;

    //TODO: show binary representations of integers

    //printing
    println!("{}", a); //display print
    println!("{:?}", b); //debug print
}

/*
float:
f32,f64
TODO: float representation IEEE- 754
TODO: float overflow and underflow

bool:
true, false
bool only needs 1 bit but since only byte is addressable so they are given 1 byte so that address with & can be taken for bool

char: 32 bits wide each - representing unicode
 */

fn floats() {
    //float
    let a = 10.8f32;
    let b = 10.8;   //default f64
}

#[test]
fn bool() {
    //bool
    let a = true;
    let b = false;
    //TODO: if we see only 1 bit is required to store bool how rust represents bool internally
    //rust takes 1 byte to store bool so that address of bool can be taken
}

#[test]
fn char() {
    //char
    let a = 'A';
    //TODO: why char is represented always in 32 bit and there UTF-8 value is not used for char- what benefit char may be simple ??
    //use unicode value to write any char
    let b = '\u{CA0}';
    //if we are in ascii range i.e. 0x0 to 0xFF (0 to 255) we can use this form also
    let c = '\x0F';
    println!("{},{}", b, c);
}

//references & pointers will have there own section

/*
&str:
array:
slice:
tuple:
struct:
enum:
 */

#[test]
fn compound_types() {
    //array
    let a = [10, 20, 30, 40, 50];
    //ref to arrays
    let a1 = &a;
    let a2 = b"Hello World";    //byte array
    //tuple
    let b = (10, 20, "Hello");
    //access tuple values
    b.0;
    b.1;
    //tuples are used for returning multiple values from functions

    //slice
    let b = &a[..]; //slice to all array elements
    let c = &a[1..3]; //slice to few elements

    let d = String::from("Hello World");
    //&str is same as slice which points to valid utf-8 collection
    //&str can point them at heap(usually written as String), stack or even in binary
    //similar to slice to array syntax &a[1..3] is not provided for &str as indexing or range getting is difficult in utf-8 which is
    //multi byte (variable size) representation
    let e = d.as_str();
}

//struct, enum etc will be taken up in separate file
