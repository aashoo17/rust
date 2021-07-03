use std::process::Command;

#[test]
fn create_process(){
    let mut a = Command::new("ls");
    //TODO: arg and env passing
    //TODO: is spawn waiting for child completion
    a.spawn().unwrap();
}