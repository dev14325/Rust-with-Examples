/*
Similar to function but function are limited , macros can be expanded
function expands during run time , macros expands during compile time 

When you use macros in Rust, the compiler expands them into regular Rust code before compilation.
cargo expand shows exactly what your macros expand into — so it’s a tool for inspecting metaprogramming results



*/


#[derive(Debug)] // procedural macro

/*
#[derive(Debug)] is a macro that automatically implements the Debug trait for MyStruct.
``
Without it, you’d have to manually write code to define how MyStruct should be printed.

*/ 
struct Temp {
    val : i32,
}


fn main(){

    let x = Temp {
        val : 12
    };

    println!("{:?}",x.val);
    let v = vec![1,2,3];
    println!("{:?}",v); // implement debug trait if we want to print like this 
}