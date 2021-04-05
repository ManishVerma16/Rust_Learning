// Option enum 
// option return either a value or no value at all
// many methods in rust return this type

fn main() {
    println!("Occupation is {}", match get_occupation("Mick") {
        Some(o) => o,
        None => "No occupation found!"
    });
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Jack" => Some("Software Developer"),
        "Mike" => Some("Singer"),
        _ => None,
    }
}

/*
Example 1 either index return something or none

let name = String::from("Rust Language");

    println!("Character at index 8: {}", match name.chars().nth(18) {
        Some(c) => c.to_string(),
        None => "No character at index 8!".to_string()
    });
    

*/