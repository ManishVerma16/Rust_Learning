fn main(){
    let mut vector: Vec<i32> = Vec::new(); // easy way to create a vector.

    let mut num_vector = vec![11,23,34,55,67];  // vector declaration and initailization

    println!("{}", num_vector[2]);

    vector.push(10);  // pushing element in vector
    num_vector.push(78); 
    println!("{}", num_vector[5]);
    println!("{}", vector[0]);

    println!("removed element {}",num_vector.remove(0));  // remove element from vector

    // printing the value of vector
    for i in num_vector.iter(){
        println!("{}", i);

    }


}