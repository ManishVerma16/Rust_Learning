// Enums are a way to express our code in descriptive and simple way where appropriate.

enum Directions {
    Up,
    Down,
    Left,
    Right
}

fn main(){
    let player_direction:Directions = Directions::Up;

    match player_direction{
        Directions::Up => println!("Player is going Up!"),
        Directions::Down => println!("Player is going Down!"),
        Directions::Left => println!("Player is going Left!"),
        Directions::Right => println!("Player is going Right!"),
    }
}

