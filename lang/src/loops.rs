pub fn for_loops() {
    for index in 0..10 {
        print!("{}, ", index);
    }

    //for loop - works with iterator
    let vector = vec![10, 20, 30, 40];
    for number in &vector {
        print!("{} ", number);
    }
}

pub fn while_loops() {
    let vector = vec![10, 20, 30, 40];

    // while loop - conditional loop
    let mut current_index = 0;
    while current_index < vector.len() {
        println!("Value at index {}: {}", current_index, vector[current_index]);
        current_index += 1;
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
