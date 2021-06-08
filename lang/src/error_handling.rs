/*
Unrecoverable Error:
panic!() macro call
this should be used when error is not recoverable
this will halt the process and unwind the stack
 */

use std::error::Error;
use std::fs::File;

#[test]
fn panic() {
    panic!();
}

/*
Recoverable Error:
Result<T,E> enum is used which has two fields Ok(T) and Err(E) like below
enum Result<T,E>{
    Ok(T),
    Err(E)
}

when error is recoverable it is used
Ok() => when this received it means success
Err() => when this happens error has happened and we can take some action
 */
#[test]
fn error() {
    let file = File::open("unavailable_file.txt");
    match file {
        Ok(_) => {}
        Err(_) => {
            //handling error
            print!("oops error was found so I am printing");
        }
    }
}

//unwrap and expect
#[test]
fn unwrap_use() {
    //if Result was found Ok() type then unwrap gives value inside Ok() otherwise panic!() is called
    let file = File::open("file.txt").unwrap();
}
#[test]
fn expect_use() {
    //if Result was found Ok() type then expect gives value inside Ok() otherwise panic!() is called with custom error message passed in  expect
    let file = File::open("file.txt").expect("file opening error");
}
// ? = used when error is received return by function
#[test]
fn return_error() -> Result<(), Box<dyn Error>> {
    //here if file is not opened Error will be returned by function
    let file = File::open("unavailable_file.txt")?;
    Ok(())
}
