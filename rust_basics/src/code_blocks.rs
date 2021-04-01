fn main(){
    let x = 10;
    {
        let y = 5;
        println!(" inner scope x: {} y: {}", x, y); // x and y both are accessible here
    }
    println!(" outer scope x: {}", x); // y is not accessible

}