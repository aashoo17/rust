//vector
#[test]
fn vector_methods() {
    //create
    let mut a = Vec::new(); //blank vector, requires type to be explicitly given, or later from push() call etc.. rust can know
    let mut b = vec![10, 20, 30, 40, 50]; //vec! macro
    let mut c = vec![0; 10]; //give a default value to all elements
    let d: Vec<i32> = Vec::with_capacity(20); //create a vector with capacity 20

    //accessing elements
    let e = &b[0]; //get reference, ownership like b[0] will destroy vector hence generally out of question
    let f = &b[0..3]; //get slice

    //iteration
    //iterate over reference
    for g in &c {
        //we get references to elements
    }
    //iterate over mutable reference
    for h in &mut c {
        //here we will get mutable reference to elements
    }
    //take ownership of elements and destroy vector
    for i in c {
        //consume the vector
    }

    //modifying vector
    a.push(10);
    a.push(20);
    a.push(30); //push at end
    a.insert(3, 40); //insert at this index

    a.pop(); //remove element from end
    a.remove(1); //remove value at this index
    a.clear(); //remove all elements

    //method call using . operator can implicitly make slice with all vector elements hence all of slice methods will work on vector too
    //TODO: give important slice methods working on vector
    //all these are slice methods working on vector implicitly
    b.first(); //get first element of slice
    b.last(); //get last element of slice
    b.get(3);
    b.first_mut();
    b.last_mut();
    b.get_mut(3);

    b.len();
    b.is_empty();

    //sorting
    b.sort(); //ascending order
    b.sort_by(|a, b| b.cmp(a)); //descending order
}
