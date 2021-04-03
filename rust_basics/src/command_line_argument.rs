// Command line argument 

use std::env;  // importing the env module

fn main(){
    // creating args variable of vector type that holds string values and holds the argument which will be passed thorugh 
    // the command line while executing this file.
    let args: Vec<String> = env::args().collect();

    // printing the arguments which is collected in the args vector.
    for arg in args.iter(){
        println!("{}", arg);
    }
}