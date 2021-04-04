// String methods

fn main(){

// Replace Method use to replace something in the string
    {
        let string = String::from("Rust is good language");

        println!("{}", string.replace("good", "interesting"));
    }
    println!("");

// Line method is used to split up the string into iterator for each line in our string.
    {
        let string = String::from("Today was very\nhot day\nof April");

        for line in string.lines(){
            println!("[ {} ]", line);
        }

    }
    println!("");

// Split method is used to split the string by a certain character or delimiter.

    {
        let string = String::from("Today+I+have+done+lots+of+work");
        // here split method returns an iterator, but we have a vector so to store them into vector we need to apply collect method to convert it into a vector of string.
        let token: Vec<&str> = string.split("+").collect();

        println!("{}", string);
        println!("Token at index 1: {} and at index 3: {}",token[1], token[3]);

    }
    println!("");

    // Trim method use to trim the string on whitespaces.
    {
        let string = String::from("  Rust  is good  language  .");

        println!("Before trim: {}", string);
        println!("After trim : {}", string.trim());

    }

    // Char at given index
    {
        let string = String::from("Rust is good language.");

        println!("{}", string);
        
        // Get character at index

        match string.chars().nth(5){
            Some(c) => println!("Character at index 5: {}", c),
            None => println!("No character at index 5: ")
        }
    }
}
