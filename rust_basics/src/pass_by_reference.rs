//  Passing value to the function by reference

struct Color {
    red: u8, // u8 -> store 0-255
    green: u8,
    blue: u8
}

fn main(){
    // Color: green, red, blue
    let bg = Color{red: 200, green: 100, blue: 50};
    print_color(&bg);

    // print_color(bg);  if we use pass by value  multiple function call cause some issue
    // print_color(bg);


}

fn print_color(c: Color){   // passing Color struct as a reference
    println!("Color is {}, {}, {}", c.red, c.green, c.blue,);

}

// if we pass bg as a value then also it works but the value of bg move out from this scope.