// copy and clone trait
/*
collections : string , vec on heap --> dont copy just borrow , ownership rule , doesn't implements copy trait , implements clone trait but expensive


int , bool -> lie on stack , can copy , implements copy trait



*/

#[derive(Debug,Clone, Copy)]

struct User {
    is_male : bool ,
    age : u32,
}

fn main() {

    let u1 = User {

    is_male : true,
    age : 23,
    };

    let u2 = u1;
    println!("{:?} , {:?}",u1,u2);

}