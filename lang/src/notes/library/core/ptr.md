# PTR

pointers in rust
1. *const T
2. *mut T
3. NonNull<T>
4. Unique<T>

## *const T 

*const T => only read the underlying value  

## *mut T
*mut T = modify the underlying value  
[lang_items like #[lang = ""]](https://doc.bccnsoft.com/docs/rust-1.36.0-docs-html/unstable-book/language-features/lang-items.html)
[lang items](https://manishearth.github.io/blog/2017/01/11/rust-tidbits-what-is-a-lang-item/)  

TODO: learn the api uses of *const T and *mut T  

## NonNull<T>
*mut T` but non-zero and covariant.

[Subtyping and Variance](https://doc.rust-lang.org/nomicon/subtyping.html)
**subtyping:**
subtyping means 1 type can be used in place of another  
for e.g. type T can be used in place where type U is required  

in rust subtyping happens over lifetimes not on any actual type 
for e.g. say we have two lifetimes  
'small and 'big  
subtyping happens when say somewhere where 'small is required we can use 'big  

and variance tells the rules about where lifetimes subtypes can be used

**variance:**

type constructor:  
A type constructor in Rust is any generic type with unbound arguments. For instance Vec is a type constructor that takes a type T and returns Vec<T>. & and &mut are type constructors that take two inputs: a lifetime, and a type to point to.  

There are three kinds of variance in Rust. Given two types Sub and Super, where Sub is a subtype of Super: 

* `F` is *covariant* if `F<Sub>` is a subtype of `F<Super>` (subtyping "passes through")
* `F` is *contravariant* if `F<Super>` is a subtype of `F<Sub>` (subtyping is "inverted")
* `F` is *invariant* otherwise (no subtyping relationship exists) 

If F has multiple type parameters, we can talk about the individual variances by saying that, for example, F<T, U> is covariant over T and invariant over U.  

|   |                 |     'a    |         T         |     U     |
|---|-----------------|:---------:|:-----------------:|:---------:|
| * | `&'a T `        | covariant | covariant         |           |
| * | `&'a mut T`     | covariant | invariant         |           |
| * | `Box<T>`        |           | covariant         |           |
|   | `Vec<T>`        |           | covariant         |           |
| * | `UnsafeCell<T>` |           | invariant         |           |
|   | `Cell<T>`       |           | invariant         |           |
| * | `fn(T) -> U`    |           | **contra**variant | covariant |
|   | `*const T`      |           | covariant         |           |
|   | `*mut T`        |           | invariant         |           |

TODO: learn why in *const T and *mut T in one T is covariant and in one it is invariant   
TODO: why in &'a T and &'a mut T => covariant over 'a in both but 1 is covariant and 1 invariant over T  


## Unique<T>

**PhantomData**
[phantom data from rust docs](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)  
well expained here  

[phantom data from rust nomicon](https://doc.rust-lang.org/nomicon/phantom-data.html)




