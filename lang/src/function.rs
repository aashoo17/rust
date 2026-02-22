fn call(){
    //do something
    println!("Hello World");
}
//function pointer
fn say(){
    let a = call;
    a();
}