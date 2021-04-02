fn main(){
    let mut string = String::from("How's it going? Ishver is here.");

    // length
    println!("Length of string: {}", string.len());

    // checkin whether string is empty or not?
    println!("String is empty? {}", string.is_empty());

    // spliting the string on whitespace
    for token in string.split_whitespace(){
        println!("{}", token);
    }

    // checking whether the string contains some value or not.
    println!("Does the string contains 'Ishver'? {}",string.contains("Ishver"));
    
    // adding something in the string.
    string.push_str(" Everything is connected...");
    println!("Length of string: {}", string);


}