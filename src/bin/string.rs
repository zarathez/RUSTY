fn main() {

    //String
    let msg: String = String::from("this is a string");
    println!("{}", msg);

    //size
    let size = msg.len();
    println!("{}", size);

    //slicing -str (str is not the same as String) 
    let msg: String = String::from("this is a string");
    let s:&str  = &msg[0..5];
    println!("{}, {}",s ,s.len() );

    //str 

    let sami:&str = "Sami";
    /*
    this will be hardcoded in the binary , so it's immutable
    */

    let multiLine:&str = r#"
    this is the first line 
    this is the second 
    "#;

    println!("{}", multiLine);

    //Deref coercion 
    let msg: String = String::from("this is a string");
    let s:&str  = &msg;
    
    //add & to String
    let mut msg: String ="Hello rust".to_string();
    msg+="!";
    println!("{}", msg );

    let first = "Hello";
    let second = "Rust";
    let msg = format!("{} , {}" , first, second);
    println!("{}", msg);

    

}