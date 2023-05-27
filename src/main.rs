fn main() {
    let my_string = String::from("Hello");

    let reversed_string: String = my_string.chars().rev().collect();

    println!("Reversed string: {}", reversed_string);
}
