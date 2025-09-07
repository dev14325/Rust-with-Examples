fn increment_vector(vec: &mut Vec<Option<i32>>) {
    // TODO
    //  Given a Vec<Option<i32>>, increment each non-None element by 1, and skip None.
 for opt in vec {
    // match opt {
    //     Some(val) =>{
    //         *val = *val * 2
    //     }
    //     None =>{
    //         ()
    //     }

    // }


    // M2 
    // if let Some(val) = opt {
    //     *val = *val * 2;
    // }

 

}
}

fn main() {
    let mut vec = vec![Some(1), None, Some(3)];
    increment_vector(&mut vec);
    println!("{:?}", vec); // [Some(2), None, Some(4)]
}
