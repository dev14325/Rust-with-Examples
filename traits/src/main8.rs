trait Licensed {
    // TODO: Add a default implementation for `licensing_info` so that
    // implementors like the two structs below can share that default behavior
    // without repeating the function.
    // The default license information should be the string "Default license".
    fn licensing_info(&self) -> String {
        return "Default license".to_string()
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line.
impl Licensed for OtherSoftware {} // Don't edit this line.

fn temp<T:Licensed>(software:&T) {
    println!("License Info : {}",software.licensing_info());

}

fn main() {
    let soft1 = SomeSoftware{version_number : 1};
    let soft2 = OtherSoftware{version_number:"2.0".to_string()};
    temp(&soft1);
    temp(&soft2);
    // You can optionally experiment here.
}