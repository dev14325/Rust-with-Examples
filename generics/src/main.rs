

// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
struct Wrapper<T> {
    value: T,
}

// TODO: Adapt the struct's implementation to be generic over the wrapped value.
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // You can optionally experiment here.
    let int_wrapper = Wrapper::new(42);
    let str_wrapper = Wrapper::new("Singh");
    let flo_wrapper = Wrapper::new(3.14);

    println!("{}",int_wrapper.value);
    println!("{}",str_wrapper.value);
    println!("{}",flo_wrapper.value);
    
}

