fn main() {
    let mut s = String::from("Ricky");
    add_last_name(&mut s);
    calculate_name_length(&s);

    println!("{}", s);
    println!("It's {} characters long", calculate_name_length(&s));
}

fn add_last_name(s: &mut String) {
    s.push_str(" Eskildsen");
}

fn calculate_name_length(s: &String) -> usize {
    s.len()
}