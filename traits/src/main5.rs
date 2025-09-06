trait Summary {
    fn summraize(&self)->String {
      return  format!("Hi there from Summary trait ...");
    }
}
trait  Fix {
    fn fixing(&self)->String {
        return format!("Hi from Fix trait..");

    }
    
}

struct  users {
    Name : String,
    Age : i32,
}
impl Summary for users {
    
}

impl Fix for users {} // 1 struct can implement multiple trait

fn notify<T : Summary + Fix>(item :T){
    println!("{}",item.summraize());
    print!("{}",item.fixing());
}
fn main() {

    let user1 = users {
        Name : String::from("Singh"),
        Age : 12
    };

    notify(user1);

}