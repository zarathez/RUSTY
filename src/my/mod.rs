// mod my {    

    use super::foo; //super means go above this module  , which mean it's the modules.rs file


    pub fn call_foo() {
        foo::print();
    }
    pub fn print() { // should be public to be used in main 
        f();
        println!("my");
    }

    fn f() { // this is a private function  
        a::print();
    }

    pub mod a ;
// }