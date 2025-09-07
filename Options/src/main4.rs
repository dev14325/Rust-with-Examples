fn double_option(opt: Option<i32>) -> i32 {
    let res = match opt {
        Some(val) =>{
        val*2
        }

        None =>{
        0
        }
    };
    return res;
}

fn main() {
    let a = Some(5);
    let b: Option<i32> = None;
    println!("{}", double_option(a)); // 10
    println!("{}", double_option(b)); // 0
}
