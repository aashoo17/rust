// [understanding the closure concept in rust](https://internals.rust-lang.org/t/still-confused-by-move-vs-closures/1430)

/*
Fn, FnMut, FnOnce
*/

//Fn closure

#[test]
fn fn_closure() {
    //a data type which move - i.e. it is not marked Copy type
    let a = String::from("Hello World");

    //this is Fn closure
    let b = || {
        //as a captured here via reference like &a - its Fn closure
        let c = &a;
        print!("{}", c);
    };

    //variable b is Fn closure so this can be called as many time we want
    b(); //think of it like b.call(&self); so closure b is taken as reference
    b();
}

//FnMut closure
#[test]
fn fnmut_closure() {
    //a data type which move - i.e. it is not marked Copy type
    let mut a = String::from("Hello World");

    //this is FnMut closure
    let mut b = || {
        //as a captured here via reference like &mut a - its FnMut closure
        let c = &mut a;
        print!("{}", c);
    };

    //variable b is FnMut closure so this can be called as many time we want
    b(); //think of it like b.call(&mut self); so closure b is taken as mutable reference
    b();
}

#[test]
fn fnonce_closure() {
    //a data type which move - i.e. it is not marked Copy type
    let a = String::from("Hello World");

    //this is FnOnce closure
    let b = || {
        //as a captured here via move like a - its FnOnce closure
        let c = a;
        print!("{}", c);
    };

    //variable b is FnOnce closure so this can be called as many time we want
    b(); //think of it like b.call(self); so closure b is consumed/moved so can not be called again
         //b();    //this will throw error
}

//TODO: I think now move has no functionality in closure - Fn, FnMut, FnOnce can describe everything even FnOnce describes that value
// captures has to be moved - so no explicit move is required at all
