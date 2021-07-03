use std::collections::LinkedList;

#[test]
fn list() {
    //create
    let mut a: LinkedList<i32> = LinkedList::new(); //blank list

    //modify
    a.push_back(60);
    a.push_front(70);
    a.push_front(80);

    a.pop_back();
    a.pop_front();
    // a.remove(2);     //unstable

    //get elements
    a.front(); //first element in the front
    a.back(); //last element in the back
    a.front_mut();
    a.back_mut();

    //length
    a.len();
    a.is_empty();

    //iteration
    //shared refs
    for i in &a {}
    //mutable refs
    for i in &mut a {}
    //consume list and take ownership
    for i in a {}
}
