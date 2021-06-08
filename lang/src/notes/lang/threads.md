# Threads

Do read following topics before threads for better understanding
closures  
ownership and borrowing  
println!() macro use  

## creating a basic thread

```rust
// importing thread module from std
use std::thread;

fn main(){
    // creating a str variable
    let str = "Hello There";
    //spawn will create new os thread
    // passing closure to spawn func
    // using move in closure to take full ownwership
    // of str variable from main thread to this
    // new thread
    let t = thread::spawn(move || {
        println!("{}",str);
    });

    // using _ to store the Result and 
    // not use it otherwise compiler will throw
    // warning
    // we could use unwrap() too
    // t.join().unwrap()
    let _ = t.join();
}
```

## passing multiple values and reading the value

```rust
use std::thread;
use std::sync::mpsc;

fn main(){
    //creating a channel which returns a tupple
    // assign it to another tupple like destructuring
    // to assign to individual values
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        //creating a vector
        let v = vec!(1,2,3);
        for x in v{
            //by looping passing multiple values
            // since it is move closure
            // takes the ownership of tx
            // in this thread
            tx.send(x).unwrap();
        }
    });
    //now rx works as iterator no need
    // to call recv() function
    for i in rx{
        println!("{}",i);
    }
}
```