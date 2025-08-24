fn main () {
    //tuple
    let t:(bool, u32 , char) = (true , 1 , 'c');
     
    //destructure 
    let (a,b,c) =t;
    println!("{}, {}, {}", a, b, c);

    //ignore with 
    let (_,b,_c) = t;
    println!("{}", b); //in case we only want b

    //empty tuple 
    let t = ();

    //nested tuple

    let nested = ((1.23 , 'a') , (true , 1u32 , 'b') , ());
    
    //tuple and nested tuple access
    let t:(bool, u32 , char) = (true , 1 , 'c');
    println!("t= {}, {}, {}",t.0 ,t.1 ,t.2 );
    println!("nested 0.0 : {}", nested.0.0 );



   
    

}