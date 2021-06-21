/*
expr, stmt, ty, pat, item, block, ident, tt:
[expr, stmt, ty, pat, item, block, ident, tt can be learned thoroughly from here](https://doc.rust-lang.org/reference/introduction.html)

built in macro:
some macros are built in rustc and we can't do much about them except from knowing their functionality and using them
file!(), line!(), column!(), concat!(), cfg!(), include!(), macro_rules!
*/

/*
writing your own macros using macro_rules:
macro_rules!  name_of_macro{
    (pattern) => (template);
    (pattern2) => (template2);
    (pattern3) => (template3);
    ....
}
TODO: explain ($l:expr,$r:expr) in pattern for MatchPatternMacro
this says we will match two expressions separated by comma first expression stored in variable $l and second in $r
*/

// a very simple macro which takes blank pattern as input
macro_rules! simple_macro {
    //here pattern is blank
    () => {
        1 + 2
    };
}

#[test]
fn test_simple_macro() {
    let a = simple_macro!();
    print!("{}", a);
}

//macros tries to match 1st pattern then 2nd and so on if none matches throws error
macro_rules! match_pattern_macro {
    //pattern 1 - says that look for 1 expression and if found name it to $l so dollar where $l is written it will be replaced by that expression
    ($l:expr) => {
        //return i32
        $l
    };
    //pattern 2
    ($l:expr,$r:expr) => {
        //return bool
        $l < $r
    };
    //pattern 3
    //different returns not generally possible in in same function signature
    //unless you internally rename them like in cpp - function overloading
    () => {
        "Return Something"
    };
}

#[test]
fn test_match_pattern_macro() {
    //example showing all three pattern in working for MatchPatternMacro
    let a = match_pattern_macro!(10);
    let b = match_pattern_macro!(2, 3);
    let c = match_pattern_macro!();
    print!("{} {} {} ", a, b, c);
}

/*
caveats in using macro:
1. storing pattern expression value into a variable once expression is evaluated rather than keep on calling that expression
2. borrowing expression rather than owning most of the time
*/
macro_rules! macro_caveats_evaluating_expression_one_time {
    ($l: expr, $r: expr) => {
        //do something with $l and $r
        let a = $l + $r;
        //do something with $l and $r again here
        let b = $l + $r;

        //usually we will expect $l and $r to have same value throughout the block and a and b to be same and can write logic based on that
        //but expression can evalute different values at different time
        //a simple example Vec.pop() - this will pop element from the back and give different result each time called
        //so expression Vec.pop() gives different result each time
        //so usually it is good practice to store value of expression like $l and $r in a variable and work on it
        //rather than evaluating them again and again
    };
}

macro_rules! macro_caveats_borrowing {
    ($l: expr, $r: expr) => {
        match (&$l: expr, &$r: expr) {
            (l,r) => {
                //do something with l and r here
                //we know that l = &$l so borrows expression value and assigns to a
                //but if l would have been owner it will pass ownership to a now and l will become useless further
                //which we want to use as many times as we want - welcome to rust world lol
                let a = l;
                let b = r;
            }
        }
    };
}

/*
repetition:
Pattern Meaning
$( ... )* Match 0 or more times with no separator
$( ... ),* Match 0 or more times, separated by commas
$( ... );* Match 0 or more times, separated by semicolons
$( ... )+ Match 1 or more times with no separator
$( ... ),+ Match 1 or more times, separated by commas
$( ... );+ Match 1 or more times, separated by semicolons
*/

//think of vec! macro which can take as many elements as it want how it is written
macro_rules! custom_vec {
    //in pattern we are using $( ... ),* this rule put as many expression separated by comma
    ($($elm:expr),*) => ({
        let mut a = Vec::new();
        //$elm here gives each expression value one by one
        //so think of it as a loop in action
        $(a.push($elm);)*
        //return the vector
        a
    });
}

#[test]
fn test_custom_vec() {
    let a = custom_vec!(10, 20, 30, 40, 50); //CustomVec![] can also be uses instead of CustomVec!()
    print!("{:?}", a);
}

macro_rules! recursion_in_macro {
    //pattern 1
    ($($l:expr),*) => {{
        let mut a = Vec::new();
        $(a.push($l);)*;
        a
    }};

    //pattern 2 using recursion - to support trailing comma
    //$($elm:expr),+ says match multiple expression at min 1 expression should be present
    //and a comma added , like this $($elm:expr),+, says you can have trailing comma
    ($($elm:expr),+,) => ({
        //here macro calls itself to match a given pattern
        let a = recursion_in_macro![$($elm)*];;
        a
    });
}

#[test]
fn test_recursion_in_macro() {
    let a = recursion_in_macro![10, 20, 30, 40];
    print!("{:?}", a);
}

/*
#[macro_use] mod macros;
if a module contains macros then we have to attach this module to tree #[macro_use] mod macros; rather than mod macros; to use macros
I think this is required as macros don't have pub like syntax to make them public
*/

/*
procedural macros :
TODO: make explanation concise and clear - I think it still not
#[derive(Clone, Copy)] etc..
procedural macros can derive traits automatically if all field of that struct/type already implements that trait
for e.g.

#[derive(PartialEq)]
struct Human{
    a: i32,
    b: i32
}

since both fields are i32 which are themselves defines PartialEq on themselves
we can derive PartialEq for Human
for this we will check equality a and equality of b
*/

#[derive(PartialEq, Clone, Debug)]
struct Human {
    name: String,
    age: i32,
}
#[test]
fn test_human_for_partial_eq() {
    let a = Human {
        name: "Human".to_string(),
        age: 20,
    };
    let b = Human {
        name: "Human".to_string(),
        age: 20,
    };

    print!("{}", a == b);
}

/*
TODO: Custom macros like #[tokio::main] :

References:
Chapter 20 from Programming Rust book - excellent for starting here
[rust macro book by danielkeep](https://danielkeep.github.io/tlborm/book/index.html)
 */

//TODO: see the implementation of the vec![] macro

//TODO: integer types are lot of them i8..isize and u8..usize if we want do something all of them
// we have to put so much code manually its better use a macro which will implement it for all of them
// and code is written once - see the core lib of rust - num module
