use std::path::Path;

/*
Drop trait:
we can use this to run our custom code when value gets dropped
*/

struct Human;

impl Drop for Human {
    fn drop(&mut self) {
        //run some code here
        print!("{}", "Something gets dropped here");
    }
}

#[test]
fn test_drop() {
    Human;
}

/*
Sized trait:
so sized means fixed memory size always
i32 - always 4 bytes
Vec<T> = fixed size (pointer size + length + capacity)
though vector has memory on heap which grows and shrink on stack we keep fixed size (usize + usize + usize)

now Sized trait is implemented by default for all types
if we want to opt out i.e. type can or can't be sized we have to use ?Sized which we would be using rather than Sized in practice
*/
struct SomeUnsizedStruct<T: ?Sized> {
    a: T,
}

/*
Clone:
a type T can create copy of itself
T to T (a new copy)
T.clone() => T

now std lib gives a generic implementation which makes it also possible
&T to T (a new copy)
&T.clone() = T
*/
#[derive(Clone)]
struct CloneStruct;

/*
Copy:
Copy trait signifies that a type will copy itself rather than moves which generally happens in rust
like at let declarations, when passed inside function args
so for a copy to be possible type T has to know how a I can create duplicate of myself
as we know Clone trait can facilitate that functionality - a Copy type has to implement Clone beforehand
*/
#[derive(Clone, Copy)]
struct SomeCopyStruct;

/*
Deref, DerefMut:
specify how dereferencing operators like * and . behave on your types
Smart pointers are classic example of this
Rc<T>, Mutex<T> etc
*/
#[test]
fn box_smart_pointer() {
    let mut a = Box::new(10);
    //struct Box is being used as pointer - DererMut example here
    *a = 20;
    println!("{}", a);
}

/*
Default:
default value of a type
TODO: what is the uses though
if we want default value call ::default()
*/
#[derive(Default)]
struct DefaultStruct;

fn default_test() {
    let a = DefaultStruct::default();
}

/*
AsRef<T>, AsMut<T>
if i say this
T: AsRef<U> => this means I can do this conversion
T to &U
similarly
T: AsMut<U> means
T to &mut T

e.g.
String: AsRef<str>
so
String to &str is possible

this will be used in function generally

fn my_func(x: &str){
    //do something
}

my_func(String::from("Hello"));
*/
//here T can be anything which implements AsRef<Path>
// in other words T can be converted to &Path
fn open<T: AsRef<Path>>(a: T) {}
/*
Borrow<T> vs BorrowMut<T>
this is same as AsRef<T> and AsMut<T>
but they solve different problem - in Hashing

e.g.
String implements AsRef<str>, AsRef<[u8]> etc that means
String to &str
String to &[u8]
is possible

but for hashing String gives same hash as &str gives same hash but &[u8] will give different hash

so rust says if two types can give same hash tell it to rust by implementing Borrow<T>, BorrowMut<T> trait
if String implements Borrow<str>
so rust is now confident that if String have Borrow implemented I will get same hash with &str also

what problem this solves

let a: HashMap<String, i32> = HashMap::new();

say my get implemented like this

this will take ownership of key if move type is there like String
impl HashMap<K, V> where K: Eq + Hash{
fn get(&self, key: K) -> Option<&V> { ... }
}

key just used for lookup we don't want ownership
impl HashMap<K, V> where K: Eq + Hash{
    fn get(&self, key: &K) -> Option<&V> { ... }
}

I have to write like this
hashtable.get(&"twenty-two".to_string())

It should be good enough to pass anything that can be hashed and compared with our key type; a &str should be perfectly adequate,
impl HashMap<K, V> where K: Eq + Hash{
fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
where K: Borrow<Q>,
Q: Eq + Hash
{ ... }
}

hashtable.get("twenty-two") can be used
*/

/*
From and Into:
from T to U => From trait on T
U into T  => Into trait on U

uses
Into used in functions to simplify args passing
fn ping<A>(address: A) -> std::io::Result<bool>
    where A: Into<Ipv4Addr>
{
    //do things
}
so type A passed as arg could be anything which can be converted into Ipv4Addr

From traits from method serves as a generic constructor for producing an instance of a type from some other single value.
let addr1 = Ipv4Addr::from([66, 146, 219, 98]);
let addr2 = Ipv4Addr::from(0xd076eb94_u32);
*/

/*
ToOwned:
with Clone trait we can do this
T to T (copy)
&T to T(copy)

what if we want
&T to U (another type)
we can do this using ToOwned - only restriction is that
U implements Borrow<T>

*/

/*
TODO: Cow - copy on write
*/
