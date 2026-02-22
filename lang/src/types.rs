fn integers() {
    let signed_8bit: i8 = 10i8;
    let signed_16bit: i16 = 10i16;
    let signed_32bit_default: i32 = 10;
    let signed_64bit: i64 = 10i64;
    let signed_arch_size: isize = 10isize;    //this will be used for values created from pointer subtraction

    //unsigned integer types
    let unsigned_8bit: u8 = 10u8;
    let unsigned_16bit: u16 = 10u16;
    let unsigned_32bit: u32 = 10u32;
    let unsigned_64bit: u64 = 10u64;
    let unsigned_arch_size: usize = 10usize;

    //octal, hex & binary
    let octal_value: i32 = 0o12;
    let hex_value: i32 = 0x12;
    let binary_value: i32 = 0b10;

    //printing
    println!("{}", signed_8bit); //display print
    println!("{:?}", signed_16bit); //debug print
}

/*
float:
f32,f64

bool:
true, false
bool only needs 1 bit but since only byte is addressable so they are given 1 byte so that address with & can be taken for bool

char: 32 bits wide each - representing unicode
 */

fn floats() {
    //float
    let float_32bit: f32 = 10.8f32;
    let float_64bit_default: f64 = 10.8;   //default f64
}

fn bool() {
    //bool
    let is_true: bool = true;
    let is_false: bool = false;
    //TODO: if we see only 1 bit is required to store bool how rust represents bool internally
    //rust takes 1 byte to store bool so that address of bool can be taken
}

fn char() {
    //char
    let letter_a: char = 'A';
    //use unicode scalar value to write any char.
    let unicode_char: char = '\u{CA0}'; //this is a kannada letter
    println!("{},{}", letter_a, unicode_char);
}

fn tuples() {
    //tuple
    let empty_tuple: () = ();   
    let single_type_tuple: (i32,i32) = (10,20);   
    let mixed_tuple: (i32, i32, &str) = (10, 20, "Hello");
    //access tuple values
    println!("{} {} ", mixed_tuple.0, mixed_tuple.1);
    //tuples are used for returning multiple values from functions
}
//array, slice , String and &str is explained in refrences
//struct, enum etc will be taken up in separate file
