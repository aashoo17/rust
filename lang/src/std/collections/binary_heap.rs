use std::collections::BinaryHeap;

#[test]
fn binary_heap() {
    //create
    let a: BinaryHeap<i32> = BinaryHeap::new();
    let b: BinaryHeap<i32> = BinaryHeap::with_capacity(20);
    let mut c = BinaryHeap::from(vec![10, 20, 30, 40, 50, 60]);

    c.push(70); //push the value
    c.pop(); //remove greatest ordered value

    let d = c.peek().unwrap(); //can know which will be popped next
}
