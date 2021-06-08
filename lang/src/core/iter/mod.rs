mod traits;

/*
Iterator
DoubleEndedIterator
Marker traits

 */

// i am not going to implement these iterators only will see uses where they are implemented already
//may be on arrays, slices etc..

#[test]
fn iterator() {
    let a = [10, 20, 30, 40, 50];
    //get the iterator
    let mut b = a.iter();
    //using next()
    assert_eq!(b.next(), Some(&10));
    //using for in loop
    for i in &a {
        print!("{},", i);
    }
}

#[test]
//count no of elements in iterator
fn count() {
    let a = [10, 20, 30, 40, 50];
    //get the iterator
    let b = a.iter();
    let c = b.count();
}

//get the last element
#[test]
fn last() {
    let a = [10, 20, 30, 40, 50];
    let b = a.iter();
    let c = b.last();
}

//advance in steps

//get nth element

//StepBy iterator - gives iterator which has steps elements

//Chain iterator - chain two iterators together

//Zip iterator - zip two iterators together so that resulting zipped iterator will give tuple as its elements
//having elements of both iterator

//Intersperse iterator

//IntersperseWith

//map adaptor

//for_each()

//Filter iterator - use predicate (closure) to filter elements

//Enumerate

//Peekable

//SkipWhile

//TakeWhile

//MapWhile

//Skip

//Take

//Scan

//FlatMap

//Flatten

//Fuse

//
