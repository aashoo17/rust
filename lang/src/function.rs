fn call(){
    //do something
    println!("Hello World");
}

//function pointer
#[test]
fn say(){
    let a = call;
    a();
}