trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.


impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());
        return self
    }
   
}
fn main() {
    // You can optionally experiment here.
    let v = vec![String::from("Foo")];
    let v = v.append_bar();
    print!("{:?}",v);


   

}


mod tests {
    use super::*; // bring AppendBar trait + impl into scope

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar"); // last element must be "Bar"
        assert_eq!(foo.pop().unwrap(), "Foo"); // first element must be "Foo"
    }

    #[test]
    fn multiple_append_bar() {
        let v = vec![String::from("Foo")]
            .append_bar()
            .append_bar()
            .append_bar();

        // After 3 calls, vector should be: ["Foo", "Bar", "Bar", "Bar"]
        assert_eq!(v, vec![
            String::from("Foo"),
            String::from("Bar"),
            String::from("Bar"),
            String::from("Bar")
        ]);
    }

    #[test]
    fn empty_vec_append_bar() {
        let v: Vec<String> = Vec::new().append_bar();
        // ["Bar"] should be the result
        assert_eq!(v, vec![String::from("Bar")]);
    }
}

