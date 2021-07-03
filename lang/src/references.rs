/*
Reference:
unlike owners which drops memory when they go out of scope
references will never do that i.e. they will not cleanup the memory when they go out of scope
but they have one big restriction that they can not be valid/used when owner which they are reference to is gone

Lifetime:
so lifetime in which reference can be safely used is lifetime of owner
so rust will automatically check that you are not bypassing that rule

in some cases though rust can not figure out if reference will be valid in that cases we have to give
explicit lifetime to tell compiler till when reference should work

restrictions on references from same owner:
1. shared refs &T can be taken as many time as we want
2. &mut T (mutable reference) can taken only one in same scope
3. &T and &mut T can not exist together is same scope
 */
#[test]
fn working_with_references() {
    //todo: here 3rd restriction that both &T and &mut t can co-exist is not getting followed
    // is it because accesses are sequential in this code and both &T and &mut T can not access data together
    let mut a = String::from("Hello World");
    let b = &a;
    let c = &mut a;
    let d = &a;
}

/*
simple types like i32, f32 which in general can be represented by type T give to reference
&T and &mut T

compound types like array, vec etc give references which are called slice and represented as
&[T]

reference to utf-8 sequence is called slice as &str in rust
&str can point to utf-8 sequence in binary, stack, heap

String works with utf-8 sequences on heap - so a slice for String is &str
*/

fn slices(){
    let a = [10,20,30,40,50];
    let b = &a[..];

    let c = String::from("Hello World");
    //todo: this will possibly will fail with utf-8
    let d = &c[..];     //I think this way it is possible because of chars being ascii only
}
