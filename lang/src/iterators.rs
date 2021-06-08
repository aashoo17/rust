/*
Iterator:
somebody who implements iterator trait can keep on emitting elements when next() call is made

IntoIterator:
collection will implements this iterator so when into_iter() method is called they will emit Iterator can be looped upon
for in will automatically call into_iter() on collection to loop
though we can pass iterator directly to for in but it is rare in practice
say we have a collection c so in general we implement IntoIterator for three variant of collection c
IntoIterator for c
IntoIterator for &c
IntoIterator for &mut c

iter() and iter_mut() call:
so for loop calls into_iter() automatically for us
but say if we do it manually on references(shared, mutable) how we will do it for a collection named c
(&c).into_iter()
(&mut c).into_iter()
this is not ergonomic or we can say looks bad in writing
so we have shorthand to write it using iter() and iter_mut()
(&c).into_iter() => c.iter()
(&mut c).into_iter() => c.iter_mut()

FromIterator:
so any collection which implements FromIterator can be created from Iterator
we call collect() method on Iterator and boom got the collection
TODO: why collect() call if FromIterator gives from_iter() call, is it a convenience method

Iterator adapters:
modify iterator and produce new iterator using closures

TODO: learn about other types of iterators also
 */

#[test]
fn iterator() {
    //lets take vector and show iterator things
    let mut a = vec![10, 20, 30, 40, 50];

    //IntoIterator
    //borrow
    for b in &a {}
    //borrow mutably
    for c in &mut a {}
    //consume
    for d in a {}

    let mut e = vec![10, 20, 30, 40, 50];

    //do it without for loop - iter() and iter_mut()
    //FIXME: fix the loop - infinitely running
    // while let Some(f) = e.iter().next(){
    //
    // }
    // while let Some(g) = e.iter_mut().next(){
    //
    // }

    //FromIterator - get vector from iterator of integers
    let h = 10..20;
    let i: Vec<i32> = h.into_iter().collect();

    //iterator adapter
    //these are used to modify Iterator and produce new one

    let j = vec![10, 20, 30, 40, 50];
    let k = j.iter().map(|a| a + 1); //new iterator which adds 1 to all elements getting produced

    for l in k {
        print!("{} ", l);
    }

    let m = j.iter().filter(|a| **a < 40);
    for n in m {
        print!("{} ", n);
    }
}
