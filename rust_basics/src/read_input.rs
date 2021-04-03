use std::io;  // importing io module

fn main(){
    let mut input = String::new();

    println!("Say something: ");

    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("You said: {}", input);
        },
        Err(e) => println!("Oops! Something went wrong: {}.",e)
    }
}