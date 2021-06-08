use std::ops::{Add, Neg};

#[derive(Debug)]
struct Human {
    age: i32,
}
impl Human {
    fn new(age: i32) -> Self {
        Self { age }
    }
}

// binary operator
//TODO: tell all traits which implements some operator
impl Add for Human {
    //+ operator returns i32 on this type
    type Output = i32;
    //how value is calculated for result of + on Human
    fn add(self, rhs: Self) -> Self::Output {
        self.age + rhs.age
    }
}

#[test]
fn test_add_operator() {
    let a = Human::new(20);
    let b = Human::new(30);
    //a and b both gets consumed in + why this behaviour was selected for Add trait
    let c = a + b;
}

//unary operator
impl Neg for Human {
    type Output = i32;

    fn neg(self) -> Self::Output {
        -self.age
    }
}

#[test]
fn test_negation() {
    let a = Human::new(20);
    let b = -a; //showing wrong type Human in place of i32 in intellij

    print!("{:?}", b);
}

/*
PartialEq, Eq, PartialOrd, Ord
What does equality stand for in programming
1. x == y then y == x
2. x == y and y == z then x == z
3. x == x

if all three are valid we call it equivalence relation and represented by Eq trait in Rust
if first two are valid then it is called Partial Equivalence and represented by PartialEq trait
3rd relation x == x fails with float (so they are a good example)

so if PartialEq is implemented on type we can do == and !=
PartialOrd if implemented on type then we can do <, <=, >, >=
for PartialOrd it is required that PartialEq is already implemented for that type

usually in a struct where we will implement PartialOrd we check all fields one by one to reach to conclusion
that who is equal
for e.g.
struct Man{
    a: i32,
    b: f32
}
if two copies of Man are present if asked are they equal we will see that both a are equal and is both b are equal
then they are == otherwise !=
we can write this logic by hand but rust can automatically do that for you using = #[derive(PartialEq)]
same goes for PartialOrd we can derive it with #[derive(PartialOrd)]

this is not always true but most of the time so go the easy route
 */

#[derive(PartialOrd, PartialEq, Debug)]
struct Man {
    a: i32,
    b: f32,
}

#[test]
fn test_equality_ord() {
    let a = Man { a: 20, b: 20.1 };
    let b = Man { a: 20, b: 20.1 };
    let c = Man { a: 30, b: 20.1 };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

/*
Index<T>, IndexMut<T>
if they are implemented we can use a[i] type of syntax for that type
this is generally used for collections - which keeps more than one element with them

for e.g. array
let arr = [10,20,30,40,50];

we can do => arr[0]
and also => arr[1..3]

so two things are implemented here behind the scenes
Index<usize> and Index<Range<usize>>
 */
