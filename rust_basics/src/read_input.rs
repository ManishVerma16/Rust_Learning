use std::io;  // importing io module

fn main(){
    let mut input = String::new();

    println!("Say something: ");

    // from io module stdin method and read_line method is called then passed to the input variable.
    // this will result in success => Ok or failure => Err
    match io::stdin().read_line(&mut input){ 
        Ok(_) => {
            println!("You said: {}", input.to_uppercase());
        },
        Err(e) => println!("Oops! Something went wrong: {}.",e)
    }
}