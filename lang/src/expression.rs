fn expression() {
    // Arithmetic expressions
    let sum = 5 + 10;

    // Block expressions (evaluates to the last statement without a semicolon)
    let y = {
        let x = 3;
        x + 1
    };

    // If expressions (used as ternary)
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // Match expressions
    let boolean_text = match condition {
        true => "Yes",
        false => "No",
    };

    // Loop expressions (returning values from break)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Results: {}, {}, {}, {}", sum, y, number, result);
}