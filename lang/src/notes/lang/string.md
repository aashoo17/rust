# String

[String from rust docs](https://doc.rust-lang.org/std/string/struct.String.html)


```rust
fn main(){
    //string creation
    let mut s = String::from("Ashutosh");
    let t = String::new();

    //UTF-8
    //multibyte characters are UTF-8 so no indexing possible
    //if utf-8 is not required OsString has to be used  

    //Deref trait implementation
    //&String => can be dereffed to &str  
    //so in place of &str in function args &String can be used


    //modifying String
    //push => for inserting char
    //push_str => for inserting &str
    s.push_str(" Singh");

    println!("{}",s);
}
```