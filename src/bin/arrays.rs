fn main() {
    let arr: [u32;3] = [1,2,3];
    println!("{:?}", arr[0]);

    //in case we want to mutate it should be declared as mut first 

    let mut arr: [u32;3] = [1,2,3];
    arr[1] = 9;
    println!("{}", arr[1]);

    //array filled with zeros 
    let mut arr: [u32;10] = [0 ; 10];
    println!("{:?}", arr);
    let mut arr: [u32;7] = [1,2,3, 4, 5, 6, 7];

    //slicing
    let s = &arr[0..3];
    println!("{:?}", s);
    //slicing it all 

    let s0 = arr;
    let s1 = &arr[..];
    println!("{:?}", s0 == s1); //this returns true 
    



}