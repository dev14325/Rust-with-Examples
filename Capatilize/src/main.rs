// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
// fn capitalize_first(input: &str) -> String {

//     let mut chars = input.chars();

//     match chars.next() {
//         None => String::new(),
//         Some(ans) => ans.to_uppercase().collect::<String>() +  chars.as_str(),
//     }


    // M2
//   if input.is_empty(){
//     return String::new();
//   }


//   let first = input.chars().next().unwrap();
//   let rest = &input[1..];

//   first.to_uppercase().collect::<String>() +  rest



   

// }

// fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
//     let mut ans = Vec::new();
//     for  word in words {
//       let mut chars = word.chars();

//    let capital_word =  match chars.next() {
//         None => String::new(),
//         Some(ans) => ans.to_uppercase().collect::<String>() +  chars.as_str(),
//     };
//     ans.push(capital_word);
//     }
//     return ans;
// }

// fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
//     words.iter().map(|w| {
//         let mut chars = w.chars();
//         match chars.next(){
//             None => String::new(),
//             Some(ans) => ans.to_uppercase().collect::<String>() + chars.as_str(),
//         }
//     }).collect()

// }

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|w|{
        let mut chars = w.chars();
        match chars.next(){
            None => String::new(),
            Some(ans) => ans.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }).collect::<Vec<String>>().join("")
    
}



fn main() {
    // You can optionally experiment here.
//     let str2 : String = String::from("ffffhello world");
//    println!( "{}" , capitalize_first(&str2));


// let s = String::from("Hello world");
// let slice = &s;
// let part = &s[0..3];
// println!("{} {} {}",s,slice,part);


// let nums = vec![1,3,4];
// let slice :  &[i32]= &nums


let words = vec!["hello" , " " , "dev"];
println!("{:?}" , capitalize_words_string(&words));
// let slice : &[&str] = &words;

// println!("{:?}",slice);


}


