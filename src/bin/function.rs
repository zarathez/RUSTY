fn add(x:u32 , y:u32) -> u32 {
    x+y
    // Whatever Statement is at the end of the function without the returnm zill be the value that will be returned
}

fn print() { //could not return anything
    println!("no output");
}


//diverge - never return

fn forever () -> ! { // `!` :this tells rust that this function will never return
    loop { }
} 

fn crash() -> ! {
    panic!("crash");
}

fn main() {
    let (x , y) =(12 , 13);
    let z = add(x,y);
    println!("{}", z);

    print();

    // forever();
    crash();





}