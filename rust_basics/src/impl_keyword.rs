// impl keyword is used to add method to a structs to make it more useful

struct Rectangle {
    width: u32,
    length: u32
}

impl Rectangle {
    fn print_description(&self){
        println!("Rectangle: {} x {}", self.width, self.length);
    }

    fn area(&self){
        println!("Area: {}", self.width*self.length);
    }

    fn is_square(&self) -> bool{
        self.width == self.length
    }
}
fn main(){
    let mut rect = Rectangle{ width: 10, length: 5};
    rect.print_description();
    rect.area();
    rect.width = 5;
    println!("Rectange is square: {}", rect.is_square());
}