use std::any::type_name;

fn get_type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

/*
    Variables
*/

const FIVE: u32 = 5;

fn vars() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is now: {x}");
}

/*
    Datatypes: Integers/Floating point, Bool, Char
*/

fn dtypes_num() {
    let num: i32 = 10;
    let pi: f64 = 3.14;

    println!("\nThis number, {}, has a type of {}, and this number, {}, has a type of {}", num, get_type_of(&num), pi, get_type_of(&pi));
}

fn dtypes_bool() {
    // inferred
    let t = true;
    
    // explicit
    let f: bool = false;

    println!("\nVariable 't' is equal to {}, and variable 'f' is equal to {}. Both are of type {}.", t, f, get_type_of(&t));
}

fn dtypes_char() {
    // char can be ascii, accented letters, chinese, japanese, korean, emoji, and zero width spaces
    
    // inferred
    let c = 'z';
    
    // explicit
    let z: char = 'Z';

    // emoji
    let heart_eyed_cat = '😻';

    println!("\nVariable 'c' is equal to {}", c);
    println!("Variable 'z' is equal to {}", z);
    println!("Variable 'heart_eyed_cat' is equal to {}", heart_eyed_cat);
    println!("All three are of type {}", get_type_of(&c));
}



/*
    Compound types: Tuple type, Array Type
*/

fn ctypes_tuple() {
    
    // you must specify the type of each item in the tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructuring a tuple
    let (x, y, z) = tup;
    println!("\nThe value of x, y, and z are: {}, {}, {}", x, y, z);
}



/*
    main function
*/

fn main() {
    vars();
    dtypes_num();
    dtypes_bool();
    dtypes_char();
    ctypes_tuple();
}