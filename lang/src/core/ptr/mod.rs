/*
[understand rust raw pointer from this](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/unsafe.html)

Raw Pointer: *const T & *mut T
- *const T = for read only access
- *mut T = for mutable access

1. raw pointers could be
- null
- unaligned (this will happen rarely)
- accessing invalid/freed memory
2. No resource cleanup afterwards - they never own anything in first place for cleanup but problem is that they can
take you to a place even if it is cleaned up
3. no ownership semantics of rust compiler for them
4. if send in multi thread using say Send trait they can access same memory from different threads same time and data races
will occur so synchronisation would be required
5. no lifetime so dangling pointer can not be seen by rust compiler for raw pointers - compiler doesn't know if pointed value
by this pointer is still vaild as compiler can do using lifetime for references &T and &mut T
6. only 1 guarantee comes - *const T will not modify underlying data ever

TODO: These are the question in my mind now
1. so how these can be assigned to null
2. so we will always get them using &T or &mut T => so how they can unaligned (is it by hardcoding address, are using
pointer arithmetic with offset() or ffi into other language like c
is there any other way ?
3. how data associated with raw pointer can be cleaned up
so as i said already pointer don't allocate resources in first place but since we will do ffi with c and dynamic memory
comes with raw pointers they may not allocated the memory but only using these pointers we can clean the underlying memory
si in a way they are responsible for resource de-allocation

My thoughts on ptr:
so pointer never allocates memory is just points to them which we can use to get the underlying value
now when we use these raw pointers we have to ensure that whoever has allocated the memory at ptr location
have not cleaned it up at the time of ptr de-referencing

single thread:
in single thread only problem is that we should not access memory by ptr when it is cleaned up that's it
and that comes with stack allocation.

multi thread:
in multi thread we have bigger problem we can access same memory location using ptr at same time from different
threads which will bring data race (this will happen for two *mut T accessing from different thread or
combination of *const T and *mut T accessing data from different thread)
TODO: put data race explanation here

now if we see dynamic memory from c they allocate memory and point it using a pointer and clean it up using that
ptr so though ptr did not allocate the memory it is only way cleanup can happen i.e. using this pointer

**Safe abstraction over raw pointers: give idea**
so it is very simple
1. create a struct over raw pointer
2. make this struct field private = now nobody can access it
3. we give safe abstractions over in form of struct methods
4. if any unsafe method is there put in unsafe block use it on your own risk
5. for memory cleanup implement Drop trait on this struct once struct is gone
from scope memory will be cleaned up using drop destructor

TODO: why casting raw pointers back to references is unsafe say *const T to &T
when raw pointer goes back to being reference
we know that references need lifetime that means till when they can used safely (for that underlying data is valid)
now when we are getting back the reference to whose lifetime we can attach these references
now in docs it says it is arbitrary and may are may not work so unsafe
now how can explicitly attach to some lifetime etc and ensure this does not ruin the memory

[pointers explained in rust](https://doc.rust-lang.org/std/primitive.pointer.html)
this is for std module though we can learn few lines for some good understanding

when *ptr is done it should be non-null and aligned
TODO: I can understand about non-null but why aligned always?
I think this is related to memory access unaligned memory takes more cpu cycles to read underlying data than aligned

getting null as ptr value
null() and null_mut() functions to get null ptr
is_null to check if value is not null
pointer arithmetic like c use offset() on *const T and *mut T

TODO: what about memory cleanup, how that works with pointer
TODO: what about memory cleanup of a type with struct wrapper around these pointers like Unique<T> and NonNull<T>

explanation which i have already written in short, still leaving here for more context

Here are some things to remember about raw pointers that are different than other pointer types. They:
- are not guaranteed to point to valid memory and are not even guaranteed to be non-NULL (unlike both Box and &);
- do not have any automatic clean-up, unlike Box, and so require manual resource management;
- are plain-old-data, that is, they don't move ownership, again unlike Box, hence the Rust compiler cannot protect against bugs like use-after-free;
- lack any form of lifetimes, unlike &, and so the compiler cannot reason about dangling pointers; and
- have no guarantees about aliasing or mutability other than mutation not being allowed directly through a *const T.

what does Unique<T> and NonNull<T> brings to table over raw pointers:




[understand rust raw pointer from this](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/unsafe.html)

 */

mod const_ptr;
mod mut_ptr;
mod non_null;
mod unique;
