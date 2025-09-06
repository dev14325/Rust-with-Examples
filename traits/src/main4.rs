pub trait temp {
  fn exp(&self)->String {
   return format!("Hi there .......");
  }   
}

struct  User {
    name:String,
    age:i32,
}

struct fix;

impl temp for User {}

impl temp for fix {}

fn notify(item:&impl temp) {
    println!("Hi , the info is {}",item.exp());


}
fn main() {
    let users = User {
     name : String::from("Singh"),
     age : 22,
    };

    // println!("{}",users.exp());

    notify(&users);
    let f = fix;
    notify(&f);
    
}  
