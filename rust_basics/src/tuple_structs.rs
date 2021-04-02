//  Tuple Structs

struct Color (u8, u8, u8);

fn main(){
    // Color: green, red, blue
    let mut red = Color(200, 100, 50);

    println!("red is {}, {}, {}",red.0, red.1, red.2);
    red.2 = 45;
    println!("red is {}, {}, {}",red.0, red.1, red.2);


}