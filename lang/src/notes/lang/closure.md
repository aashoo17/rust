[understanding rust closure](https://internals.rust-lang.org/t/still-confused-by-move-vs-closures/1430)

let x = || {println!("Hello")};  

this statement creates instance of struct having data and method  

//Ashu and call are my given names rust will create names in its own way 

```rust
struct Ashu{ 
    //variables
}
impl Ashu{
    //call can take &self or &mut self
    fn call(&self){
        println!("Hello")
    }
}
```
now this struct will be initialized in memory and the method can access struct mutably or immutably

## capturing environment  
normally functions can access values created inside it or passed as argument  

but closures can access the value in the same scope  

```rust
fn main(){
    let y = 1;
    //y will be accessed in the closure even after it is not available in the x declaration of closure  
    //y can be passed as &y or &mut y (if y is mutable itself)
    //giving ownership to closure
    //move will pass ownership of y to closure and closure can consume y
    let x = || {println!("{}",y)}; 
}
```

