pub mod a { //nested module , must be declared public if u want to use the module directly from outside my::a
    #[derive(Debug)]
    pub struct S {
        pub id : u32, // this is a private field even if the whole struct is public 
        pub name: String,
    }
    pub fn print()  {
        println!("a");
    }

    use super::super::foo; //two super because we have two layers 
    pub fn call_foo() {
        foo::print();
    }

}