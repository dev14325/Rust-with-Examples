// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {

    let mut chars = input.chats();

    match chars.next() {
        None => String::new(""),
        Some(ans) => ans.to_uppercase().collect::<String>() +  chars.as_str(),
    }
   

}

fn main() {
    // You can optionally experiment here.
    let str : String = String::from("hello world");
   println!( "{}" , capitalize_first(&str));

}


