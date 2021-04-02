struct Person{
    name: String,
    age: u8
}

impl ToString for Person {
    fn to_string(&self) -> String{
        return format!("My name is {} and I am {}.", self.name, self.age);
    }
}
fn main(){
    let ish = Person { name: String::from("Ishver"), age: 21};

    println!("{}",ish.to_string());
}