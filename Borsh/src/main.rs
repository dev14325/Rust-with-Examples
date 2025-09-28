use borsh::{BorshSerialize,BorshDeserialize};

#[derive(BorshDeserialize,BorshSerialize,Debug,PartialEq)]
struct Temp {
    id : u32,
    data : String,
    v : Vec<u32>
}


fn main() {
    let t = Temp {
        id : 20,
        data : String::from("I'm Great"),
        v : vec![1,9,23]
    };

    let mut buffer :Vec<u8> = Vec::new();
    t.serialize(&mut buffer).unwrap(); // converts struct into bunch of bytes
    println!("{:?}",buffer);

    // Deserialzation

    let user_info = Temp::try_from_slice(&buffer).unwrap(); // Deserialization needs to know which type to reconstruct from the byte
    println!("{:?}",user_info);
    
}
