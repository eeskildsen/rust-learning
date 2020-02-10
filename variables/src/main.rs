fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("Here's another way of accessing the value of y: {}", tup.1);

    // Add 2 to 2
    println!("I'm pleased to announce that 2 + 2 = {}", add_two_to_two());
}

fn add_two_to_two() -> i32 {
    2 + 2
}