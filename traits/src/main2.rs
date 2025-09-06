use std::fmt::format;

pub trait temp {
 fn exp(&self) ->String  {
   return format!("Hi there");
 }   
}

struct  User {
    name:String,
    age:i32,
}

impl temp for User {
   
}
fn main() {
    let users = User {
     name : String::from("Singh"),
     age : 22,
    };

    println!("{}",users.exp());
    
}  
