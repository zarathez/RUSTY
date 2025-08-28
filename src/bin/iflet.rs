#![allow(unused)]

fn main() {
    let x: Option<u32> = None;

    match x {
        Some(v) => println!("Some {v}"),
        _ => {}
    }

    //if let , if we used if alone , the Some(v)=x won't compile . the only alternative use is match 
    // you have to start with Some(v) , because : if x = Some(v) attempts assignment (which returns (), not a boolean).
    // Some(v) = x is called a `pattern matching syntax` (where the pattern goes on the left side of =), it's only allowed in if let 

    if let Some(v) = x {
        println!("if let {}",v );
    }

    let Some(v) = x else {
        //the code diverge here 
        panic!("x is none");

    };

    //let-else follows let statement syntax (because it starts with let) so it should end with a semicolon
    // if let follows if expression syntax (because it starts with if) so no semicolon
    // It's about which syntactic family they belong to

    println!("v= {}",v );

}