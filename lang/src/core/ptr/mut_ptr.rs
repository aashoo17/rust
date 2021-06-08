//*mut T
//similar api to *const T only few differences

#[test]
pub fn mut_ptr() {
    //create
    let a = &mut 20 as *mut i32;
    let b: *mut i32 = &mut 20;

    //check null
    let c = a.is_null();
    assert_eq!(c, false);

    //cast
    let d = a.cast::<u32>();

    //get references &T and &mut T
    let e = unsafe { a.as_ref() }.unwrap();
    let f = unsafe { a.as_mut() }.unwrap();

    //offset i.e. pointer arithmetic like c
    let g = unsafe { a.offset(4) };
    //TODO: why wrapping offset is safe and how it works differently than offset()
    let h = a.wrapping_offset(4);
    let i = unsafe { a.offset_from(b) };
    let j = unsafe { a.add(4) };
    let k = a.wrapping_add(4);
    let l = unsafe { a.sub(4) };
    let m = a.wrapping_sub(4);

    //guaranteed equality and non-equality
    //TODO: need to understand them more how they are different from == and <,> etc created using traits

    let n = a.guaranteed_eq(b);
    assert_eq!(n, false);
    let o = unsafe { a.guaranteed_ne(b) };
    assert_eq!(o, true);

    //read and write
    //TODO: difference with read, write and de-referencing using *
    let p = unsafe { a.read() };
    unsafe { a.write(20) };

    //copy to and from pointer location
    //these copies will work even if overlapping memory is there
    unsafe { a.copy_to(b, 1) };
    unsafe { a.copy_from(b, 1) };

    //if sure not overlapping memory
    unsafe { a.copy_from_nonoverlapping(b, 1) };
    unsafe { a.copy_from_nonoverlapping(b, 1) };

    //drop
    unsafe { a.drop_in_place() };
}

//access a memory after cleanup in function in single thread
pub fn after_cleanup_single_thread_access() {
    let a = access_freed_memory();
    unsafe {
        //accessing memory and modifying which was cleaned up
        *a = 20;
    }
}

fn access_freed_memory() -> *mut i32 {
    let mut a = 20;
    let b = &mut a as *mut i32;
    b
}

//show multi thread data race
pub fn multi_thread_data_race() {
    /*
    so *mut T does not implement Send trait so it can be send in different thread
    TODO: may we can send *const T into one thread and keep *mut T in main thread to see data race

        let mut a = 10;
        let b = &mut a as *mut i32;
        thread::spawn(||{
            unsafe {
                *b = 20;
            }
        });
        thread::spawn(||{
            unsafe {
                *b = 30;
            }
        });
        */
}

pub fn multi_thread_data_race2() {
    /*
    ok so we can not share the *const T also in new thread as Send not implemented
    let mut a = 10;
    let b = &mut a as *mut i32;
    let c = &a as *const i32;

    //deref the value
    unsafe {
        *b = 20;
    }
    thread::spawn(|| unsafe {
        let d = *c;
    });
     */
}

//ok so only way we can send raw pointers in new thread using wrapper over raw pointers and make them Send
//TODO: is Unique<T> and NonNull<T> anyone is Send
