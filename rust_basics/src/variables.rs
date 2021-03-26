/* Introduction to let and mut keyword */

fn main() {
    /*
        A variable in rust can be created by using let keyword.
        Normal Variable defined is immutable in rust.
    */
    let x = 45;
    println!("The value of x is {}",x);

    // to make a variable mutable use 'mut' keyword after let keyword.
    let mut y = 60;
    println!("The value of y is {}",y);

    y = 10;
    println!("The value of y is changed to {}",y)


}
