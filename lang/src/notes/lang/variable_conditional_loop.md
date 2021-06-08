# Variables
u8 = this is used for handling any raw byte based things  
i32 = common int type  
usize = this is 32 bit/64 bit based on processor type this becomes useful when when we want to convert memory address in int type and do int type calculation as memory addresses does not allow more than ++/-- arithmetic.  
```rust
//int
let a: i32 = 10;
let b : u8 = 20;
let c : usize = 30;
//float
let d: f64 = 10.0;
//bool
let e : bool = true;

//borrow
let mut f : i32 = 20;
//immutable borrow
let g : &i32 = &f;
//mutable borrow
let h : &mut i32 = &mut f;
```

## Arrays and Slices
```rust
//i32 array
let a: [i32;5] = [10,20,30,40,50];

//u8 array
let b : [u8;5] = [10,20,30,40,50];

//String on heap
let c = String::from("Hello World");

//check the type when borrowing arrays vs slices
//borrow arrays
let d: &[i32;5] = &a;
let e: &[u8;5] = &b;

//slices
let f: &[i32] = &a[..];
let g: &[u8] = &b[..];
```


