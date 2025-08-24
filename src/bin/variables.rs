#![allow(unused)]

fn main() {
    let x:i32 = -123;
    //x+=1;
    //this will not comile because x is immutable

    let mut y:i32 = -123;
    y+=1;
    
    let z =-123;
    // this will also not throw an error because the comiler will assign a type to it 

    // let w:() = -123.21; this is a trick to know the type of anything , u just comile 
    // and you will find  "expected `()`, found floating-point number" , so it's a floating-point number 

    const NUM:u32= 1;
    let x:i32 = -1;
    let x:bool = true; // it's called shadowing , u can redeclare a variable

    let v: Vec<_> =vec![1,2,3];

} 