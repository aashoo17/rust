use std::collections::{BTreeSet, HashSet};

#[test]
fn hashset() {
    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::with_capacity(20);
    let mut c: HashSet<i32> = vec![10, 20, 30, 40, 50].into_iter().collect();

    //size
    c.len();
    c.is_empty();
    c.contains(&10);
    c.insert(60);
    c.remove(&10);

    //iteration
    for d in &c {}
    for e in c {}
    //TODO: set based operations union, intersection etc..
}

#[test]
fn btreeset() {
    let a: BTreeSet<i32> = BTreeSet::new();
    let b: BTreeSet<i32> = BTreeSet::from(a);
    let mut c: BTreeSet<i32> = vec![10, 20, 30, 40, 50].into_iter().collect();

    //size
    c.len();
    c.is_empty();
    c.contains(&10);
    c.insert(60);
    c.remove(&10);

    //iteration
    for d in &c {}
    for e in c {}
    //TODO: set based operations union, intersection etc..
}
