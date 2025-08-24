#![allow(unused)]
    enum Animal {
        Cat,
        Dog,
        Duck,
        Mouse,
    }


fn main() {

    //match
    let x =1;
    match x  {
        1 => println!("one" ),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    //multiple cases 
    match x {
        1 | 2 | 3 => println!("one or two or three"),
        _ => println!("other"),
    }

    //range 
    match x {
        1..=10 => println!("between one in 10"),
        _ => println!("other"), //mandatory 
    }
    //range to know exactly what matched
    match x {
        i @ 1..=10 => println!("between one in 10 , matched {i}"),
        _ => println!("other"),
    }

    //return value 
    let animal = Animal::Cat;

    let animalSound = match animal {
        Animal::Cat => "meow",
        Animal::Dog => "Woof",
        Animal::Duck => "quack",
        _ => "?",

    };

    println!("animal sound : {}", animalSound );

    //Options

    let o : Option<i32> = None;
    match o {
        None => "None",
        _ => "response",
    };

    let x :Option<i32> = Some(1);
    match x {
        Some(v) => println!("Some({v})"),
        None => println!("None"),
    }

    
}