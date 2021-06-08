# Vector

[vector from rust docs](https://doc.rust-lang.org/std/vec/struct.Vec.html)

```rust
fn main(){
    //vector
    //mutable access to modify vector
    let mut v = vec![1,2,3];
    //creating empty vector type info has to be explicit here as empty vector does not provide type deduction
    let w: Vec<i32> = Vec::new();

    //push and pop
    v.push(10);
    v.push(20);

    //pop() gives optional so we can use while let when Some() will be received we will loop for any other case  
    //we will break the loop
    while let Some(z) = v.pop(){
        println!("{}",z);
    }
    //indexing Index and IndexMut trait  
    //Index trait enables use to write this form x[index] but read only access => k = v[1] (reading value)
    //IndexMut gives modifiable access => v[1] = 1 (modifying value)
    v[1] = 1;
    //slicing
    //slices are read only for vector they can't modify vector  
    //they are best suited for passing into function as argument when read only access is required from a vector
    //single element slice
    let m = &v[2];
    //multi element slices
    let n = &v[1..3];

    println!("{:?}",v);
}
```