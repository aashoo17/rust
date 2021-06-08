# Data types

Numbers - integers, floats
characters
boolean

## Integers
**how integer is represented at machine level**  
[Decimal and binary conversion of int](https://www.wikihow.com/Convert-from-Decimal-to-Binary)  
int representation was very easy any no can be represented in binary form as described above.
more bytes mean bigger no can be represented 

**processor working on bytes**  
this pretty much explains how processor fetches the memory  
this memory is kept in registers say take a 64 bit register ( common in x86_64 processors)  
byte kept in register is 00000000 00000000 00000000 00000000 00000000 00000000 00100000 10000000  (big endian)
now processor can operate on 1st byte only/first 2 bytes combined/ 4 bytes/  all 8 bytes

see intel manual volume 1 chapter 3.4.1 table 3.2 Addressable General Purpose Registers
that is why we have type like i8, i16, i32, i64 exists compiler will know which opcode to use
for e.g. we have i8 then use opcode which operates on 1 byte of register

usually processor will fetch 32/64 bits, but they can work on 8/16/32/64 bytes.
so we have types based on it
i8,i16,i32,i64,i128,isize   - signed types
u8,u16,u32,u64,u128,usize   - unsigned types

then came negative no, so the easy way to do it would have been keep 1 bit for sign say  
0000 0001 = 1
1000 0001 = -1 (8th bit is 1 showing -ve no)

but 0 is problem here we get two 0 as +0 and -0 then  
0000 0000 = +0
1000 0000 = -0

[2's complement system](https://en.wikipedia.org/wiki/Two%27s_complement)  
[2's complement conversion with examples](https://en.wikibooks.org/wiki/A-level_Computing/AQA/Paper_2/Fundamentals_of_data_representation/Twos_complement)
show came 2's complement, in this we reverse the bits and 1 to no to get the negative no of it  
0000 0001 = 1  
1111 1110 (bits reversed)
1111 1111 = -1 (1 is added)  

this way we will not get two zeros  
0000 0000 = 0  
get 2's complement  
1111 1111 (reverse bits)  
0000 0000 (add 1) => so we get same byte for 0's 2's complement  

so in language there is no -2 representation of bytes  
here - acts as unary operator and gets the 2's complement bits  
2 = 0000 0010  
-2 = 1111 1110 (- operator gets the 2's complement)  

TODO: why addition/subtraction for signed and unsigned types are same processor doesn't have to do anything
to differentiate between these two types (signed and unsigned)
same opcode is used for add/sub even if signed or unsigned type

**explicit integer conversion**  
so rust does not allow implicit conversions, so
TODO: let a: i8 = 10  (is it explicit conversion to i8)
TODO: are these literal also have type 10i8, 10i32 etc..

**Integer Overflow** 
The unsigned integer j is acting like a car’s odometer. When it reaches its maximum value,
it starts over at the beginning. The integer i acts similarly. The main difference is that the unsigned int variable j,
like an odometer, begins at 0, but the int variable i begins at –2147483648. Notice that you are not informed that i has
exceeded (overflowed) its maximum value. You would have to include your own programming to keep tabs on that.
The behavior described here is mandated by the rules of C for unsigned types. The standard doesn’t define how signed types
should behave. The behavior shown here is typical, but you could encounter something different
//TODO: how does this work at byte level

**octal and hexadecimal**  
octal = starts with 0 like 0o28  
hex = starts with 0x like 0xAE35
binary = start with 0b like 0b01010111

## Characters
[ascii table for char](http://www.asciitable.com/)  
so for characters it was the easiest thing to do give each character a no
ascii was first standardised version to be used everywhere but since it was using 8 bits it reached its limitation 
that it can not represent every possible characters
so came the unicode - use 32 bit rather than 8 bits 
but using 32 bit for a single character is ok but say for string og character with each 32 bits this will take 4 times the space than ascii
so unicode came with many encoding most famous of them is utf-8
which can use 1 byte also for a character and even upto 4 bytes when required and compatible with ascii
because all these features it was started to be used everywhere - rust also uses it for string


### float/double
[IEEE-754 float visualization](http://bartaz.github.io/ieee754-visualization/)

[How can cpu determine the data type of a byte?](https://cs.stackexchange.com/questions/45794/how-does-a-computer-determine-the-data-type-of-a-byte)  
so write a program with 1 signed and 1 unsigned type vs both unsigned type and add them see their assembly which addition command is used. is it 
different for signed types

TODO: 2's complement allows that same addition/substraction can be done on these low level bytes without processor worrying about sign type 
(signed or unsigned). How ??

TODO: what are the arithmetic operations for which type is signed/unsigned will matter, see the intel manual and find out


**Floating-Point Overflow and Underflow**  
This is an example of overflow—when a calculation leads to a number too
large to be expressed. The behavior for this case used to be undefined, but now C specifies that toobig gets assigned a special value that
stands for infinity and that printf() displays either inf or infinity (or some variation on that theme) for the value.  
There will be a number that has the smallest possible exponent and also the smallest value that still uses all the bits available to represent
the mantissa. This will be the smallest number that still is represented
to the full precision available to a float value. Now divide it by 2. Normally, this reduces the exponent, but the exponent already is as small
as it can get. So, instead, the computer moves the bits in the mantissa over, vacating the first position and losing the last binary digit.
This situation is called underflow, and C refers to floating-point values that have lost the full precision of the type as subnormal. If you 
divide by a large enough value, you lose all the digits and are left with 0. The C library now provides functions that let you check whether
your computations are producing subnormal values.  

NaN, or not-a-number  
asin() function a value, and it returns the angle that has that value as its sine. But the value of a sine can’t be greater than 1,
so the function is undefined for values in excess of 1. In such cases, the function returns the NaN value, which printf() displays as nan, NaN,
or something similar.

## Compound types - Array, Vector, Slice, String, &str
**array**  
arrays are nothing but collection of same types of elements stored in memory continuously together.  
type of array in rust is [T;N]  
arrays keeps their memory on stack  
so they can not grow once created only they can be modified  

**Vector**  
they are same as array - collection of same types of elements stored in memory continuously together  
but vector keeps its memory on heap and on stack they have pointer to heap memory  
so vector can grow as elements on heap can grow  
type - Vec<T>

**Slice**  
slice are pointer/reference to a collection of elements on stack or heap - they are fat pointers, so they keep pointer + length of memory  
slice can point to full arrays/vector or to a part of arrays/vector  
type - &[T]  
they are references, so they do not cleanup memory when goes out of scope  

arrays and vector implement AsRef<[T]> trait which guarantees when required this conversion is possible
[T;N] to &[T]  
&[T;N] to &[T]  
Vec<T> to &[T]  
&Vec<T> to &[T]  
so many times we will see that method implemented on slice will also work on array and vectors  

**String**  
string is same as vector storing u8 bytes which are valid utf-8 sequence on heap  
so string can grow/shrink - all things heap can do  
so vector of u8 guaranteeing that they are utf-8 is string  

**&str**  
they are same as slice to collection of valid utf-8 collections  
user can keep utf-8 values at three places
1. in binary executable (which are read only marked by OS/kernel)  
2. on stack
3. on heap (as string does)

so &str can point to all these 3 places  
we will see though as common use case it will point to memory of string (which allocates on heap)  

```rust
    fn main(){
        let a = "Hello"; //this is &str and memory gets stored in executable so can't be modified
    }
```

## How computer fetches memory and memory alignment
[Data/memory alignment: why it is required](https://developer.ibm.com/technologies/systems/articles/pa-dalign/)

## what is a variable at hardware level

for simplicity if we think some address from RAM is stored in one of the registers say RSP   
then we can locate any other address just by adding a no to it  
RSP+1, RSP+10, RSP+200 etc..

so a variable at low level is pretty much something/data held at an address, 
but we don't get any way to share this address   
so variable say x then only x can manipulate data at that address  

pointer/references is another kind of variables which can share the addresses  
so more than 1 pointer can access same memory  
hence pointer is pretty much developed for sharing of data  

we can have struct, union, enum at high level to point to data at some memory location also  

## How stack and stack frames works at hardware level in x64

![stack and stack frames](assets/stack_&_stack_frames.png)  

think of these 2 scenarios in c  

a function call = hello()  defined as void hello(){}  
so all local variables of functions and some args (as defined by abi of OS/hardware) are stored on stack memory   
just as stack is created RBP and RSP registers point to starting of stack  
once something is pushed on stack RSP is moved to point to next memory location  
when function returns we move RSP back to RBP location  

and another fn call = greet() defined as void greet(){hello();}  
so here greet will work same as hello() function before but  
once hello is called new stack is to be created these small stacks of functions are called as   
stack frames and entire stack frame memory can be seen as stack  

now before moving RSP when hello() is called we move RBP at this new call location of hello() so  
that when hello() returns we can come back to to stack frame of greet()  
but new problem arises as since RBP was modified how we will know where greet() stack frame was started  
so somehow we have to push the RBP address before incrementing RSP when hello() was called  

see in assembly   

greet:
    
hello:
    push %rbp    #push rbp address first
    # do something else


## data type conversion say an int to short etc. 
so these type of conversions are really supported by hardware so they are not specific to any language but depends on underlying processor  

References:-
TODO: put intel manual and amd64 manual link
