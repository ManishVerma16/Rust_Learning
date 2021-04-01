// tuples are basically just a bunch of variables in one collection
// Tuples can store the data of multiple types.
// Tuples are used for destructing assignments which means declare multiple variables at once using tuples.


fn main(){
    let tup1 = (10,12, 32, 40, 41); // tuple of integers.
    println!("{}", tup1.1);  // accessing tuples element using . dot and then index of the element.
    println!("{}", tup1.2);
    println!("{}", tup1.4);
}

/*

let tup1 = (10,12, 32, 40, 41); // tuple of integers.
    println!("{}", tup1.1);  // accessing tuples element using . dot and then index of the element.
    println!("{}", tup1.2);
    println!("{}", tup1.4);

Tuple having multiple datatype values

let tup1 = (20, "Rust", 3.4, false, (1, 4, 7)); // tuple of different datatypes.
    println!("{}", tup1.1);  // accessing tuples element using . dot and then index of the element.
    println!("{}", tup1.2);
    println!("{}", (tup1.4).2); // accessing the nested tuples element.

    
Example of destructing assignments.
let tup1 = (20, "Rust", 3.4); 
    let (a,b,c) = tup1;
    println!("{}", a);  
    println!("{}", b);
    println!("{}", c);


*/