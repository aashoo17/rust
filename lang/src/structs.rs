#[derive(Debug)]
pub struct Person {
    pub name: String, //pub - makes name public
    age: i32,         // private by default.
}
/*
self: Self => `self` is the instance of the struct the method is being called on. Takes ownership of the instance.
&self: &Self => `&self`: Borrows the instance immutably.
&mut self: &mut Self => `&mut self`: Borrows the instance mutably.
 */
impl Person {
    // Associated function to create a new `Person` instance.
    fn new() -> Self {
        Person {
            name: String::from("Some random name"),
            age: 20,
        }
    }
    /// Takes an immutable reference to self (&self). Used for reading data without ownership.
    fn get_age(&self) -> i32 {
        self.age
    }
    /// Takes a mutable reference to self (&mut self). Allows modifying fields of the struct.
    fn change_age(&mut self) {
        self.age = 30;
    }
    /// Consumes the struct (self), taking ownership.
    /// The original instance is moved and can no longer be used in the caller's scope.
    fn consume_struct(mut self) -> Self {
        self.age = 40;
        return self;
    }
}

//accessing struct field
pub fn access_person_struct_fields() {
    let person_instance = Person::new();
    println!("{:?}",person_instance)
}

// A tuple-like struct.
#[derive(Debug)]
pub struct Bounds(usize, usize);

pub fn bounds() {
    let image_bounds = Bounds(1024, 768);
    println!("{:?}, {}, {}",image_bounds,image_bounds.0,image_bounds.1)
}

// A unit-like struct, often used for marker traits.
struct Marker;

impl Marker {
    // A method for the unit-like struct.
    fn info(&self) {
        println!("This is a unit-like struct.");
    }
}

fn test_create_marker() {
    let m = Marker;
    m.info();
}

// A generic struct holding a type `T`.
struct Container<T> {
    name: String,
    age: i32,
    value: T,
}

impl<T> Container<T> {
    // Returns a reference to the 'value' field.
    fn get_value(&self) -> &T {
        &self.value
    }
}

impl Container<i32> {
    // Method available only for `Container<i32>`.
    fn special_i32_method(&self) {}
}

// A struct using lifetimes to hold a reference.
struct RefWrapper<'a> {
    label: &'a str,
    count: i32,
}

#[derive(Copy, Clone, Debug)]
// Demonstrates common derived traits.
struct Config {
    is_active: bool,
    id: i32,
}

// A custom trait definition.
trait RandomTrait {
    fn custom_func();
}

// Implementation of a custom trait for `DeriveTrait`.
impl RandomTrait for Config {
    fn custom_func() {
        println!("Custom function called for Config");
    }
}

fn test_bounds() {
    bounds();
}
