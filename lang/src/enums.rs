//simple C - style enum
enum MyOrdering {
    Less,    //Less gets value 0 by default
    Equal,   //= 1
    Greater, //= 2
}

enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

//enums with data
#[derive(Debug)]
enum Human {
    Name(String),
    Age(i32),
}

//generic enums
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

enum MyOption<T> {
    Some(T),
    None,
}

//pattern matching with match
fn pattern_matching() {
    let a = MyOrdering::Equal;
    match a {
        //if enum value is Less
        MyOrdering::Less => {}
        MyOrdering::Equal => {}
        //for any remaining values do this
        _ => {}
    }

    let b = Human::Age(20);
    match b {
        //ref takes the reference only to whatever String is inside Name()
        //here c: &String
        Human::Name(ref c) => {
            print!("{}", c)
        }
        //_ tells any i32 value in age() run the code - does _ takes ownership also
        Human::Age(_) => {
            print!("got age");
        }
    }
}

//matching multiple possibilities
#[test]
fn match_multiple_possiblity() {
    let a = 10;
    match a {
        1..=10 | 30..=100 => {}
        200..=300 => {}
        _ => {}
    }
}

/*
pattern guard:
If a pattern moves any values, you can’t put a guard on it. The guard
might evaluate to false, and then Rust would go on to the next pattern
 */
#[test]
fn pattern_guard() {
    let mut a = Human::Age(10);

    a = Human::Name(String::from("heman"));

    match a {
        // If a pattern moves any values, you can’t put a guard on it. The guard
        // might evaluate to false, and then Rust would go on to the next pattern
        //Fixme: try to showcase difference between ref and without ref
        Human::Name(ref name) if name.is_empty() != true => {
            print!("non empty string found");
        }
        //i32 is Copy type no ref required
        Human::Age(age) if age < 30 => {
            print!("age below 30");
        }
        _ => {}
    }
}

/*
@ patterns:
x @ pattern matches exactly like the given pattern, but on
success, instead of creating variables for parts of the matched value, it
creates a single variable x and moves or copies the whole value into it.
 */
#[test]
fn at_patterns() {
    let a = Human::Name("Hello".to_string());

    match a {
        name @ Human::Name(_) => {
            print!("{:?}", name)
        }
        Human::Age(_) => {}
    }
}
