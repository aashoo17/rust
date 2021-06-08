/*
traits:
A trait is a feature that any given type may or may not support. Most often, a trait represents a capability: something a type can do

traits should be in scope otherwise its method are hidden on implemented type
 */

/*
trait objects:
different types can implement trait that means traits will ultimately have different size
so they can not be used directly we have to use them behind pointer like &Trait which is fat pointer
rust letter said use dyn keyword - &dyn Trait (no actual change happened only visual for user)

dyn signifies that there will be dynamic dispatch here - learn how/why dynamic dispatch happens
 */

struct Lace;

impl Human for Lace {
    fn is_human(&self) -> bool {
        print!("{}", "yes i am human");
        true
    }
}

trait Human {
    fn is_human(&self) -> bool;
}
#[test]
fn test_trait_objects() {
    //using trait object is this function as argument
    fn trait_object(x: &dyn Human) {
        x.is_human();
    }
    //call trait_object() with &Lace though &Human was required
    let lace = Lace;
    trait_object(&lace);
}

/*
Generic functions:
using trait bound
TODO: tells advantages/disadvantages of using trait objects vs generic with trait bound
 */
fn generic_with_trait_bound<T: Human>(x: &T) {
    x.is_human();
}

#[test]
fn test_generic() {
    let lace = Lace;
    generic_with_trait_bound(&lace);
}
