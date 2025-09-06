use std::collections::HashMap;
fn main() {
    let mut h :HashMap<String,i32> = HashMap::new();
   h.insert(String::from("Dev"), 20);
    h.insert(String::from("Singh"), 19);
     h.insert(String::from("beast"), 26);

     let h1 = h.get("Singh");
     println!("{}",h1.unwrap());  
}
