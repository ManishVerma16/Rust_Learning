// match operator in rust. It is rust equivalent to switch statement

fn main(){
    let num = 16;

    match num {         // match -> switch statement
        1 => println!("this is one!"),      // case 1 => execution block
        2 => println!("this is two!"),     // case 2 => execution block
        10 | 11 => println!("this is either 10 or 11!"),  // checking for multiple cases
        12...20 => println!("greater than 12 and less than 20!"),  // checking for multiple cases in this range
        _ => println!("It doesn't match")  // default case
    }

    let name = "jay";

    match name {
        "jay" => println!("Hello jay, whats up!"),
        "Ajay" => println!("Hello Ajay, whats up!"),
        _ => println!("Hello Anonymous, whats up!"),
    }
}