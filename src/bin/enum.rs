#[derive(Debug , PartialEq)]
//println!("{}", Color::Red == Color::Green ); will throw a compilation error unless partialEq is activated 

enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8 , u8 , u8 , f32),
    Hex(String),
    Hsl {h:u8 , s:u8 , l:u8}
} 

fn main() {

    let color : Color = Color::Red;
    let color:Color = Color::Green;
    let color:Color = Color::Rgba(0,0,2,0.2);
    let color:Color = Color::Hex("#ffffff".to_string());
    let color:Color = Color::Hsl{h:0 , s:1 , l:200};

    println!("{:?}", color );

    println!("{}", Color::Red == Color::Green );
    println!("{}", Color::Red == Color::Red );

    //Enum of type Option :  = Some(11) | None 

    let x:Option<i32> = None;
    let x:Option<i32> = Some(-11);
    println!("{:?}", x );

    //Result = ok(10) | Err("div by 0") 

    let res: Result<u32 , String> = Ok(5);
    let res: Result<u32 , String> = Err("DIV BY 0".to_string());
    println!("{:?}", res);






}