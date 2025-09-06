trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
     return format!("{}Bar",self);
    }

}

fn main(){
let s = String::from("Foo");
let s = s.append_bar();
println!("{}",s);


}