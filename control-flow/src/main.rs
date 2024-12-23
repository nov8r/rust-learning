/*
    Control flow
*/



/*
    Control Flow: if Expressions, if else Expressions
*/

// This functions demonstrates if statements
fn if_func() {
    let number = 3;

    // condition to be met must be a bool otherwise it'll throw an error
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// This function demonstrates if else statements
fn ifelse_func(num: i32) {

    if num % 4 == 0 {
        println!("Number is divisible by 4");
    } else if num % 3 == 0 {
        println!("Number is divisible by 3");
    } else if num % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}

fn iflet_func() {
    // because if is an expression, we can use it on the right side of a let statement
    let condition = true;

    // both outcomes must be of the same type in the if expression
    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {number}")
}



/*
    Control-Flow: Loops
*/

fn loop_func() {
    /*
        loop {
            println!("I'm looping all over the place!");
        }

        the above code will continue to print the statement over and over until the user stops it
    */

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn looplabel_func() {
    let mut count = 0;
    'counting_up: loop {            //'counting_up is the label of the loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {                      // This loop has no label
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;              // this breaks out of the unlabeled loop
            }
            if count == 2 {
                break 'counting_up; // this breaks out of the labeled loop, 'counting_up.
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_func() {
    let mut number = 3;

    while number != 0 {         // A while loop runs until a certain condition is met, in our case it runs until number is = to 0
        println!("{number}!");

        number -= 1;
    }

    println!("KabooM!");
}

fn for_func() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    // for loop iterates through every element in array a
    // for every element in a, it prints out the value is with the value of the element.
    for element in a {
        println!("The value is {element}");
    }
}

fn for_range_func() {
    for number in (1..4).rev() {    // .rev() reverses it so that it counts down from 3
        println!("{number}");
    }
    println!("bazinga!");
}

fn main() {
    if_func();
    ifelse_func(7);
    iflet_func();
    loop_func();
    looplabel_func();
    while_func();
    for_func();
    for_range_func();
}
