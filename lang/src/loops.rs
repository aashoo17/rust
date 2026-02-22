pub fn for_loops() {
    for i in 0..10 {
        print!("{}, ", i);
    }

    //for loop - works with iterator
    let a = vec![10, 20, 30, 40];
    for element in &a {
        print!("{} ", element);
    }
}

pub fn while_loops() {
    let a = vec![10, 20, 30, 40];

    // while loop - conditional loop
    let mut index = 0;
    while index < a.len() {
        println!("Value at index {}: {}", index, a[index]);
        index += 1;
    }
}

fn loops(){
    // loop (infinite loop with break)
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            break;
        }
        println!("Counter: {}", counter);
    }
}

