fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
    println!("Here's another way of accessing the value of y: {}", tup.1);

    // Add 2 to 2
    println!("I'm pleased to announce that 2 + 2 = {}", add_two_to_two());

    // Return a value from a loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // Iterate over an array
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Here's a value from the array: {}", element);
    }

    // Iterate over a range
    for number in (1..22).rev() {
        println!("Here's a number from the range: {}", number);
    }
}

fn add_two_to_two() -> i32 {
    2 + 2
}