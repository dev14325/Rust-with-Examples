
fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    // let mut ans = Vec::new();
    // ans.push(input);
    // ans.push(" World");
    // ans.join("")

    let parts = vec![input ," World"];// vector of &str
    // parts.into_iter().collect::<String>()
    //  .collect::<String>() expects a iterator
    
    parts.iter().map(|x|*x).collect::<String>()

    
}

fn main() {
    // You can optionally experiment here.

    let s = String::from("Hello");
   let ans =  compose_me(&s);
   println!("{}",ans);

}