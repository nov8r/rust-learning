// This function converts from fahrenheit to celsius
// Formula: (F - 32) * 5/9 = C
fn f_to_c(temp: i32) {
    let temp = (temp - 32) * 5/9;

    println!("{temp}C");
}

// This function converts from celsius to fahrenheit
// Formula: (C * 9/5) + 32 = F
fn c_to_f(temp: i32) {
    let temp = (temp * 9/5) + 32;

    println!("{temp}F");
}

// This function produces the nth fibonacci value
// Formula: Fn = F n-1 + F n-2
// Base case: n = 0 return n, n = 1 return n
fn fib(nth: i32) {
    if nth == 0 || nth == 1 {
        println!("{nth}");
        return;
    }

    let mut prev = 0;
    let mut current = 1;

    for _ in 2..=nth {
        let next = prev + current;
        prev = current;
        current = next;
    }

    println!("{current}");
}

// This function is my unoptimized original bad christmas carol function
fn christmas_carol() {
    let items: [&str; 12] = ["A partridge in a pear tree.", 
                             "Two turtle doves,", 
                             "Three french hens,", 
                             "Four calling birds,", 
                             "Five golden rings,", 
                             "Six geese a-laying", 
                             "Seven swans a-swimming,", 
                             "Eight maids a-milking,", 
                             "Nine ladies dancing,", 
                             "Ten lords a-leaping,", 
                             "Eleven pipers piping,", 
                             "Twelve drummers drumming,"];
    
    let mut index = 1;
    let mut curr_day: i32;

    for item in items {
        if index == 1 {
            println!("On the {index} day of christmas,");
            println!("My true love gave to me");
            println!("{item}")
        } else {
            curr_day = index - 1;
            println!("\nOn the {index} day of christmas,");
            println!("My true love gave to me");

            while curr_day != -1 {
                println!("{}", items[curr_day as usize]);
                curr_day -= 1;
            }

        }
        index += 1;
    }
}

// This function is my optimized new good christmas carol function
fn optimized_christmas_carol() {
    let items = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three french hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for day in 1..=12 {
        println!("\n On the {day} day of Christmas");
        println!("My true love gave to me");

        // print gifts
        for gift in (0..day).rev() {
            if day > 1 && gift == 0 {
                println!("And {}", items[gift]);
            } else {
                println!("{}", items[gift]);
            }
        }
    }
}

fn main() {
    f_to_c(32);
    c_to_f(0);
    fib(7);
    optimized_christmas_carol();
}
