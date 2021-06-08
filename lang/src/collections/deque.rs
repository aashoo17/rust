use std::collections::VecDeque;

#[test]
fn deque() {
    //create
    let a: VecDeque<i32> = VecDeque::new(); //blank deque
    let mut b = VecDeque::from(vec![10, 20, 30, 40, 50]); //from vector
    let c: VecDeque<i32> = VecDeque::with_capacity(20); //create with initial given size

    //modify
    b.push_back(60);
    b.push_front(70);
    b.insert(2, 100);

    b.pop_back();
    b.pop_front();
    b.remove(2);

    //get elements
    b.front(); //first element in the front
    b.back(); //last element in the back
    b.front_mut();
    b.back_mut();

    //length
    b.len();
    b.is_empty();

    //iteration
    //shared refs
    for i in &b {}
    //mutable refs
    for i in &mut b {}
    //consume deque and take ownership
    for i in b {}
}
