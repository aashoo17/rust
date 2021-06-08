use core::num::NonZeroI32;
use core::num::NonZeroUsize;

//non zero nos
#[test]
fn non_zero_no() {
    //create them integers known to be non zero
    let a = NonZeroI32::new(10).unwrap();
    let b = NonZeroUsize::new(10).unwrap();

    //what if 0 is used to construct them we get None
    let c = NonZeroI32::new(0);
    assert_eq!(c, None);

    //from NonZero type get the primitive type
    let d = a.get();

    //get leading and trailing zero in binary form
    let e = a.leading_zeros();
    let f = a.trailing_zeros();
    println!("{},{}", e, f);

    //TODO: division and remainder
    // let g = NonZeroI32::new(14).unwrap();
    // let h = a/g;
    // println!("{}",h);

    //TODO: power of 2
}

//int macros
#[test]
fn int_macros() {
    let a: i32 = 20;

    //count 1 in binary form
    let b = a.count_ones();
    //count zeroes in binary form
    let c = a.count_zeros();
    //leading and trailing zeros in binary form
    let d = a.leading_zeros();
    let e = a.trailing_zeros();

    //leading and trailing ones
    let f = a.leading_ones();
    let g = a.trailing_ones();

    //Shifts the bits to the left/right by a specified amount, `n`,
    //wrapping the truncated bits to the beginning of the resulting
    let h = a.rotate_left(10); //TODO: can it be more than 32 ? for i32's which have only 32 bits
    let i = a.rotate_right(10);
}

//TODO: uint macros

//TODO: float - f32 & f64

//TODO: bignum
