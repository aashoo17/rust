use std::collections::{BTreeMap, HashMap};

#[test]
fn hashmap() {
    //create
    let a: HashMap<i32, i32> = HashMap::new();
    let mut b: HashMap<i32, i32> = HashMap::with_capacity(20);
    //HashMap implements FromIterator so it can be constructed with iterator of tuple values
    //enumerate changes vec iterator into (0,10),(1,20),(2,30),(3,40),(4,50) on which calling collect will create HashMap
    let mut c: HashMap<usize, i32> = vec![10, 20, 30, 40, 50].into_iter().enumerate().collect();

    //size and key value
    c.len();
    c.is_empty();
    c.contains_key(&10); //why reference to key
    c.get(&10);
    c.get_mut(&10); //get mutable ref

    //modify map
    c.insert(20, 200);
    c.remove(&10);
    b.clear(); //clear all items
}

#[test]
fn btreemap() {
    //create
    let a: BTreeMap<i32, i32> = BTreeMap::new();
    let mut b = BTreeMap::from(a);
    let mut c: BTreeMap<usize, i32> = vec![10, 20, 30, 40, 50].into_iter().enumerate().collect();

    //size and key value
    c.len();
    c.is_empty();
    c.contains_key(&10); //why reference to key
    c.get(&10);
    c.get_mut(&10); //get mutable ref

    //modify map
    c.insert(20, 200);
    c.remove(&10);
    b.clear(); //clear all items
}
