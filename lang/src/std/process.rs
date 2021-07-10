use std::process::Command;

//create process don't wait for child
#[test]
fn create_process(){
    let mut a = Command::new("sleep");
    a.arg("3").env("path", "10");     //arg & env passing
    a.spawn().unwrap();  //does not wait for child completion
}

#[test]
//wait for child process
fn wait_child_process(){
    let mut a = Command::new("sleep");
    a.arg("3").env("path", "10");     //arg & env passing
    a.output().unwrap();    //waits for child process to complete
}