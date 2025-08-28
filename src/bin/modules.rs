/*
======== how to organize the modules ========



*/

#![allow(unused)]
//Modules
// mod foo {
//     pub fn print() { 
//         println!("foo");
//     }
// }

// WE ORGANIZED THESE FILES 

// mod my {    

//     use super::foo; //super means go above this module  , which mean it's the modules.rs file


//     pub fn call_foo() {
//         foo::print();
//     }
//     pub fn print() { // should be public to be used in main 
//         f();
//         println!("my");
//     }

//     fn f() { // this is a private function  
//         a::print();
//     }

//     pub mod a { //nested module , must be declared public if u want to use the module directly from outside my::a
//         #[derive(Debug)]
//         pub struct S {
//             pub id : u32, // this is a private field even if the whole struct is public 
//             pub name: String,
//         }
//         pub fn print()  {
//             println!("a");
//         }

//         use super::super::foo; //two super because we have two layers 
//         pub fn call_foo() {
//             foo::print();
//         }

// }
// }



use hello-rust::my;


fn main() {
    my::print();
    my::a::print();

    let s: my::a::S = my::a::S {
        id :1 ,
        name : "S".to_string(),
    };

    println!("{:?}", s);

    my::call_foo();
    my::a::call_foo();
}