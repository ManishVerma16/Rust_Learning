use std::fs::File;   // struct to use File
use std::io::prelude::*;  // struct to perform read write operation

fn main(){

    // expect method execute and print the message if something error happen other the execution of called method.

    let mut file = File::open("test.txt").expect("Can't open file!");  // creating file object from structs and open that file.

    let mut contents = String::new();  // creating a mutable string called content

    file.read_to_string(&mut contents).expect("Oops! Can not read the file...");  // reading the content of the file using read_to_string method

    println!("File content:\n\n{}", contents);   // printing the content
}

