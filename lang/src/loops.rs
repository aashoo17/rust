#[test]
fn loops(){
    let mut i = 0;
    while i < 10 {
        print!("{} ",i);
        i += 1;
    }
    println!("\n");

    //for loop - works with iterator
    let a = vec![10,20,30,40];
    for b in &a {
        print!("{} ",b);
    }
}