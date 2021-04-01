// shadowing

fn main(){
    let mut x = 10;
    {
        let x = 15;  // shadowing -> value of changes to 15 
    println!("x: {}", x); // x having 15

    }
    println!("x: {}", x); // x having 10

    let x = "x is a string";
    println!("x: {}", x); // x changes to string

    let x = true;
    println!("x: {}", x); // x changes to bool

}