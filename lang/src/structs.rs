/*
Struct:
types of struct
1. named field
2. tuple like
3. unit like

TODO: struct layout in memory
 */
 
/*
named field struct:
convention - struct starts with caps and uses CamelCase, fields & methods are lowercase
structs are private to module use pub to make them public
even struct fields are private by default they need to be made public using pub is are to be used
 */

//using pub struct is made public - if rust module is used
pub struct Human {
    //struct field is made public
    pub name: String,
    //field is private even if struct is made public - in other modules
    age: i32,
}

/*
methods with impl - &self, &mut self, self and Self
Self is type which represents the struct type inside the impl block

&self is shorthand for self: &Self similarly
&mut self => self: &mut Self
self => self: Self
 */
impl Human {
    //associated function usually called static methods in other languages
    //Self type in return just means Human - using Self is better as if later we want to change name of struct in future
    //from Human to say NewHuman we don't have to change method signature now
    fn new() -> Self {
        Human {
            name: String::from("Some random name"),
            age: 20,
        }
    }
    //method which borrows the struct - self: &Self or self: &Human
    fn read_age(&self) -> i32 {
        self.age
    }
    //method which mutably borrows the struct - self: &mut Self or self: &mut Human
    fn change_age(&mut self) {
        self.age = 30;
    }

    //consume struct - self: Self or self: Human
    fn consume_struct(mut self) -> Self {
        self.age = 40;
        return self;
    }
}
//accessing struct field
#[test]
fn access_human_struct_fields() {
    let a = Human::new();
    assert_eq!(a.age, 20);
    assert_eq!(a.name, String::from("Some random name"));
}

// tuple like struct
pub struct Bounds(usize, usize);

fn bounds() {
    //create tuple struct
    let image_bounds = Bounds(1024, 768);
    //access its fields
    assert_eq!(image_bounds.0, 1024);
    assert_eq!(image_bounds.1, 768);
}

/*
tuple vs named field struct - which to use
internally in memory they are same so there is no benefit in terms of performance level eventually
so its for programmer clarity/ usefulness
use the one whom programmer think clear/better in writing in code in the context
 */

/*
unit struct:
so they can have only one type of instance all the time for themselves
method can be many though on them - but in terms of modification what they will do
TODO: where they get used though
 */
struct OneSuch;

impl OneSuch {
    //implement as many method as you want
}

fn create_onesuch() {
    let o = OneSuch;
}

//generic struct

struct Man<T> {
    name: String,
    age: i32,
    other: T,
}

/*
implementing methods on struct is little weird at first 
TODO: why impl requires <T> to be written if Man<T> is already written
*/
impl<T> Man<T> {}

/*
so it is possible to restrict T to only i32 and now I can implement method
so if impl Man<T> is written rust thinks T is known type like i32 for e.g. but its not so throws error
so one more T at impl is required to say it is generic type T now already known type
*/
impl Man<i32> {}

/*
lifetime in struct
I am not sure why struct requires this lifetime
I can argue for e.g. say I have 3 references fields in struct
so struct can not outlive all three - if it does for any one also it will be dangling pointer
so rust can choose smallest of all three lifetime and can compare if struct is bypassing it and that will be it
I don't have to hard code it everywhere
TODO: span which 'a represents what is it
*/

struct AnotherMan<'a> {
    name: &'a str, //lifetime is required
    age: i32,
}

//deriving traits for struct
#[derive(Copy, Clone, Debug)] //I can create all these traits on struct like magically
struct DeriveTrait{
    derive: bool,
    age: i32
}

trait RandomTrait{
    fn custom_func();
}

impl RandomTrait for DeriveTrait{
    fn custom_func() {
        //do something here
    }
}
