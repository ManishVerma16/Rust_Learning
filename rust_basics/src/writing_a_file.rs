// Creating and Writing a file

use std::fs::File;  // struct to use file
use std::io::prelude::*;  // struct to perform read and write

fn main(){
    // creating a variable to store the newly create file called output.txt
    // expect execute when some error happen
    let mut file = File::create("output.txt").expect("Could not create file!");
    
    // write_all takes content which we want to add in the file.
    file.write_all(b"This is new file created using write all method in rust.").expect("Cannot write to the file.");
}