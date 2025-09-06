// struct Point<T> {
//     x : T,
//     y : T,
// }

// fn main() {

//     let i:Point<i32> = Point { x: 12, y: 12};
//     let f:Point<f64> = Point { x: 1.2, y: 3.2 };
//     let s:Point<&str> = Point { x: "Singh", y: "Beast" };

//     println!("integer values are {} and {} ",i.x,i.y);
//      println!("float values are {} and {} ",f.x,f.y);
//       println!("Strings values are {} and {} ",s.x,s.y);

 
// }

struct Options<A,B,C> {
    x : A,
    y : B,
    z : C,
    k :  B,
}

fn main() {

let op1 = Options{x : 12 , y : 2.3 , z : "njw" ,k : 3.21};
println!("values are {} , {} , {} and {} ",op1.x,op1.y,op1.z,op1.k)
}


