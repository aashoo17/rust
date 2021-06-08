[io for rust by example](https://doc.rust-lang.org/rust-by-example/std_misc/file.html)
[file io basics in rust](https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x/31193386)

file data structure is called File in rust
brought to scope by 
use std::fs::File


opening file for read/write 
associated methods open & create
open and create methods creates File struct with values of field initialized
open will return a Result wrapped File struct opened for reading
create will return a Result wrapped File struct opened for writing it also does truncation
TODO: open/create take Path as input but I have pass &str see the significance
how &str is getting converted to Path struct

opening file for any custom mode opeartion
use std::fs::OpenOptions
[check openOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)

read/write operation
read()
write()
they are working on bytes
mutable buffers in rust i.e. slices used for storage during read/write calls

TODO: how to do the operation on char
read_lines() creating iterator over lines of file

TODO: Buffred reading/writing with BufReader/BufWriter
they are wrapper around normal readers and writers which provides buffering


closing file 
they are automatically closed or dropped when variable (File) pointing to them is gone out of scope

```rust
use std::fs::File;
use std::io::Read;

fn main() {
    //creating a mutable buffer of size 32 initialized with 0
    let mut buf : [u8;32] = [0;32]; 
    //call open to get File struct initialized
    let mut f = File::open("str.txt").unwrap();
    //read
    f.read(&mut buf).unwrap();
    println!("{:?}",buf);
}
```