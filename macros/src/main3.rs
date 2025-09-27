use std::fmt::{write , Debug , Display};
// procedural macro

#[derive(Debug)] 
// debug and display trait exist but debug macro exist , display macro doesn't exist

struct Temp {
    val : i32,
    name : String,
}

impl Display for Temp {
    fn fmt(&self ,f :&mut std::fmt::Formatter)-> std::fmt::Result {
        write!(f,"This is the struct with name {}",self.name)
    }
}


fn main(){

    let x = Temp {
        val : 12,
        name : String::from("Dev"),

    };

    println!("{:?}",x); // implement debug trait if we want to print like this 

    println!("{}",x)
    /*
    here x is a struct x doesnt implement debug trait by deafult debug micro come into the picture but we print
    like this println!("{:?}",x.name) it will work because x.name is a string and string impl debug trait 
    string , number all of that do this by default , debug trait and debug macro both are diff
    
    output of above : Temp { val: 12, name: "Dev" } ugly way , not good for user experience but 
    best for debugging
    
     */
  ; 
    // println!("{}",v);// implement display trait if you want to print like this , pretty way to print for end user like we can do v.age 
}