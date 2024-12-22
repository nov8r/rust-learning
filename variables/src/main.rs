/*
    use statement and helper function
*/

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
    println!("The constant 'FIVE' is equal to {}.", FIVE)
}



/*
    Datatypes: Integers/Floating point, Bool, Char
*/

fn dtypes_num() {
    let num: i32 = 10;
    let pi: f64 = 3.14;

    println!("\n\n----------Integer Datatype----------");
    println!("The variable 'num' is equal to {}, and has a type of {}.", num, get_type_of(&num));
    println!("The variable 'pi' is equal to {}, and has a type of {}.", pi, get_type_of(&pi));
}

fn dtypes_bool() {
    // inferred
    let t = true;
    
    // explicit
    let f: bool = false;

    println!("\n\n----------Boolean Datatype----------");
    println!("Variable 't' is equal to {}, and variable 'f' is equal to {}. Both are of type {}.", t, f, get_type_of(&t));
}

fn dtypes_char() {
    // char can be ascii, accented letters, chinese, japanese, korean, emoji, and zero width spaces
    
    // inferred
    let c = 'z';
    
    // explicit
    let z: char = 'Z';

    // emoji
    let heart_eyed_cat = '😻';

    println!("\n\n----------Char Datatype----------");
    println!("Variable 'c' is equal to {}", c);
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

    let tup2: (i32, f64, i32) = (234, 9.88, 8);

    // destructuring a tuple
    let (x, y, z) = tup;
    let (a, b, c) = tup2;

    // accessing a tuple element
    let two_thirty_four = tup2.0;
    let nine_point_eight_eight = tup2.1;
    let eight = tup2.2;
    
    println!("\n\n----------Compound Types: Tuple----------");
    println!("(Tuple: `tup`) The value of x, y, and z are: {}, {}, {}", x, y, z);
    println!("(Tuple: `tup2`) The value of a, b, and c are: {}, {}, {}", a, b, c);
    println!("(Tuple: `tup2`) Accessing element one of tuple: {}", two_thirty_four);
    println!("(Tuple: `tup2`) Accessing element two of tuple: {}", nine_point_eight_eight);
    println!("(Tuple: `tup2`) Accessing element three of tuple: {}", eight);
}

fn ctypes_array() {
    
    // unlike tuples, all items in an array have to be the same type
    // arrays also always have a fixed length
    let a1: [i32; 5] = [1, 2, 3, 4, 5];

    // if you want to fill an array with multiple of the same value,
    // you can simplify it by writing it like this:
    let a2 = [3; 5];

    // accessing array elements
    let first = a1[0];
    let second = a1[1];
    let third = a1[2];
    
    println!("\n\n----------Compound Types: Array----------");
    println!("(Array: a1) Accessing the first element of a1: {}", first);
    println!("(Array: a1) Accessing the second element of a1: {}", second);
    println!("(Array: a1) Accessing the third element of a1: {}", third);
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
    ctypes_array();
}