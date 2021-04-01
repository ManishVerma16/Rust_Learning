fn main(){

    let animals = vec!["Rabbit", "Dog", "Cat"];

    for (index, a) in animals.iter().enumerate() {
        println!("index {} and animal is {}", index, a);
    }
}


/* for loop
for i in 1..101 {
        println!("number is {}", i);
    }



let number = 30..51;  //number is a variable and 30..51 is a range

    for i in number {
        println!("number is {}", i);
        
    }

    // iterating over a vector

    let animals = vec!["Rabbit", "Dog", "Cat"];

    for a in animals.iter() {
        println!("{}", a);
    }

    // enumerate() to print the index of each element of the vector as well
    // tuple packing
    let animals = vec!["Rabbit", "Dog", "Cat"];

    for (index, a) in animals.iter().enumerate() {
        println!("index {} and animal is {}", index, a);
    }
*/