#[derive(Debug)]
struct Point {
    x :f32, 
    y :f32,
}

struct Point3d(f32 , f32 , f32);

struct Empty;

#[derive(Debug)]
//Each `#[derive(Debug)]` 
// only applies to the specific struct it's written above, 
// so `Point` and `Circle` each need their own since they're separate structs.
struct Circle {
    center: Point,
    radius: u32,
}



fn main() {

    let p:Point =Point{x:1.0 , y:2.0};
    println!("{}, {}", p.x, p.y);

    let p = Point3d(1.0,2.0,3.0);
    println!("{}, {}, {}",p.0 ,p.1 ,p.2 );

    let empty = Empty;
    let circle:Circle =Circle{ center : Point{x:1.0 , y:2.0} , radius: 2};
    println!("{:?}", circle );

    //shortcut 
    let p0 = Point{x:1.0 , y:5.7};
    let p1 = Point{x:2.0, ..p0}; // it will take the other values from p0

    //updating a struct
    let mut p = Point{x:0.0 , y:0.0};
    p.x +=1.0;
    p.y +=1.0;
    println!("{:?}", p);

}