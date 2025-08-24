#![allow(unused)]

fn main() {
    let mut i = 0;

    loop {
        println!("loop is in {}" , i);
        if i == 5 {
            break;
        }
        i+=1;
    }

    let mut i =0;
    while i<=3 {
        println!("while {i}");
        i+=1;
    }

    for i in 0..5 {
        println!("for loop {}", i );
    }

    for i in 0..=5 {
        println!("for loop {}", i );
    }

    let arr = ['a','b','c'];
    for a in arr {
        println!("{}", a);
    }

    let n = arr.len();
    for i in 0..n {
        println!("{}", arr[i]);
    }

    //for loop vector 
    let v = vec![10,20,30];
    for x in v {
        println!("{}", x);
    } 
    //if u want to loop through a vector u can only do it once due to ownership ,
    //try to copy paste and copile and it'll throw an error 
    //So if u want to iterate again , here the way : v.iter() on every vector instance ,
    //if iter is used only once it'll throw an error as well, it should be used accross the code 

    // for x in v.iter() {
    //     println!("{}", x);
    // }

    //Return value (only works for loop)
    let mut i =0;
    
    let z =loop {
        if i == 3 {
            break 99;
        }
        i+=1;
    };

    println!("z is equal to {}", z);

    // labels

    for i in 0..5 {
        for j in 0..5 {
            println!("first nested loop {i}, {j}" );
            if i==1 && j==1 {
                break ; // this will only break out of the inner loop ( it's going to stop after 1.0 1.1 , but then it's continue)
            }
        }
    }

    //that's why we have labels 

    'outer :for i in 0..5 {
        'inner :for j in 0..5 {
            println!("second nested loop    {}, {}", i,j );
            if i==1 && j==1 {
                break 'outer ; // this will only break out of the inner loop (it's gonna stop entirely after 1.1 )
            }
        }
    }


}