
use serde::{Deserialize , Serialize};
#[derive(Serialize,Deserialize,Debug)]

/*
Serialization = converting a struct (Rust object) → JSON string.

Deserialization = converting a JSON string → struct.


*/
struct User {
    username :  String ,
    password :  String
}

fn main() {

      // -------- Serialization --------
    let u = User {
        username : String::from("Singh"),
        password : String::from("123123")
    };
     

     
    let serialize_string = serde_json::to_string(&u).unwrap();
    println!("Serialized json : {}",serialize_string);
    // match serialize_string {
    //     Ok(val) => println!("{}",val),
    //    Err(_) => println!("Error Come")
        
    // }

    // -------- Deserialization --------

    let deserialized_output : User =  serde_json::from_str(&serialize_string).unwrap();
    println!("Deserialized Struct ----  username : {} , password : {}",deserialized_output.username,deserialized_output.password);



    
   
}  