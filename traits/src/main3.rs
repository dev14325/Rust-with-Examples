pub trait temp {
 fn exp(&self) ->String ;    
}

struct  User {
    name:String,
    age:i32,
}

impl temp for User {
  fn exp(&self)->String {
   return format!("Name is {} and Age is {}",self.name,self.age)
  }    
}
fn main() {
    let users = User {
     name : String::from("Singh"),
     age : 22,
    };

    println!("{}",users.exp());
    
}  
