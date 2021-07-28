#[test]
fn integers() {
    //putting i8, i16 etc at the end of literal so rust recognizes it to be i8,i16 otherwise it will thinks as i32
    //or we can explicitly write the type like these let a: i8 = 10;
    //signed integers
    let a = 10i8;
    let b = 10i16;
    let c = 10;
    let d = 10i64;
    let e = 10isize;    //this will be used for values created from pointer subtraction

    //unsigned integer types
    let f = 10u8;
    let g = 10u16;
    let h = 10u32;
    let i = 10u64;
    let j = 10usize;

    //TODO: show binary representations of integers

    //octal, hex & binary
    let k = 0o12;
    let l = 0x12;
    let m = 0b10;

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
    println!("{},{}", a,b);
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
    println!("{} {} ",b.0,b.1);
    //tuples are used for returning multiple values from functions

    //slice
    //slices has size known implicitly - not in the type
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
