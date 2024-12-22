/*
    To start a function we type `fn` followed by the function name
    in my case it's `example_function` followed by parenthesis
    and then a set of brackets so it knows where the function begins and ends.
*/
fn example_function() {
    println!("I am a function.")
}

/*
    Functions: Parameters
*/

// You must always define the type of the parameter
fn function_value(x: i32) {
    println!("The value of x is: {x}");
}

// the main funciton is the entry point for programs in rust
// meaning the program starts with this function
fn main() {
    println!("Hello, world!");
    
    // here we call the example function we defined above
    example_function();
    function_value(78);
}