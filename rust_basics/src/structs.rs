// Struct is a way to group similar bits of information into one sort of central data type.

struct Color {
    red: u8, // u8 -> store 0-255
    green: u8,
    blue: u8
}

fn main(){
    // Color: green, red, blue
    let mut bg = Color{red: 200, green: 100, blue: 50};

    println!("{}, {}, {}",bg.red, bg.green, bg.blue);

    bg.blue = 70;
    println!("{}, {}, {}",bg.red, bg.green, bg.blue);

}