#![allow(unused)]

fn main() {
    // i(n) range is from "- 2**(n-1) to 2**(n-1) - 1 "
    let i0:i8 = 0;
    /** the range here is -2**7 to 2**7-1 which is basically -128 to 127
    let i0:i8 = 128; will throw the following error 
    note: the literal `128` does not fit into the type `i8` whose range is `-128..=127`
    */

    // u(n) range is from "0 to 2**(n-1) - 1 "
    let u0:u8 = 0;

    /* the range here is 0 to 2**8-1 which is basically 0 to 255
    let u0:u8 = 256; will throw the following error : 
     the literal `256` does not fit into the type `u8` whose range is `0..=255`
    */    
    

    let f0:f64 = 0.01;
    let f1:f32 = 0.0325;
    /**/

    let b:bool = true;

    //characters 
    let c:char = 'c'; // it's a must to use single quote 
    let c:char = '♦'; //accepts emojis 

    //type conversions 

    let x:u32 = 15;
    let y:i32 = -15;

    // let z = x+y; will ERROR : the trait `Add<i32>` is not implemented for `u32`

    let z = (x as i32) +y; // this will work because u32 belongs to i32 , the other way around will cause an overflow

    //min and max values

    let i32_min:i32 = i32::MIN;
    let i32_max:i32 = i32::MAX;


    println!("the min of i32 is {i32_min} , and the max is {i32_max}");

    //overflow  
    let mut u:u32 = u32::MAX;
    u+=1;
    println!("{u}"); 

    /*
    this will fail in debug mode (cargo run --bin integer.rs).
    but will pass in release mode (cargo run --bin  --release integer.rs)
    because in release builds prioritize performance and it will not check 
    However , if we want to check , we can use the prebuilt ckecked_add
    */

    let mut u:u32 = u32::MAX;
    let u = u32::checked_add(u32::MAX , 1);
    println!("{u:?}"); // this returns None 

    let u = u32::wrapping_add(u32::MAX , 1);
    println!("{u:?}"); // this returns None 
    
    




    
}