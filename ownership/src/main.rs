fn main() {
    let mut s = String::from("Ricky");
    add_last_name(&mut s);

    println!("{}", s);
}

fn add_last_name(s: &mut String) {
    s.push_str(" Eskildsen");
}