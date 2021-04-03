// Hash Maps is just a collection of key value pairs.

use std::collections::HashMap;

fn main(){
    let mut marks = HashMap::new();

    // Adding values to the Hash Map using insert method

    marks.insert("Python", 50);
    marks.insert("C++", 40);
    marks.insert("Dart", 50);
    marks.insert("Java", 50);

    // finding the length of Hash Map

    println!("No. of languages {}", marks.len());

    // finding the value of the key
    // here using match either we find the value for the key or may be key not exist.
    match marks.get("Python") {
        Some(marks) => println!("You got {} in Python", marks),
        None => println!("Marks not found!"),
    }

    // Removing a value
    marks.remove("Java");
    println!("No. of languages {}", marks.len());

    // Loop through the Hash Map

    for (language, marks) in &marks{
        println!("In {} you got {} marks", language, marks);
    }

    // Checking for key
    println!("Are you interested in Java? {}", marks.contains_key("Java"));

}