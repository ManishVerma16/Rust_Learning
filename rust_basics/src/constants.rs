// contants are the variable that are declared in global socpe and cannot be changed.
// constants are declared with all letter in Uppercase and spacing is signified by undersocres then the data type of constant variable

const MAXIMUM_NUMBER:u8 = 20;

fn main() {
    for n in 1..MAXIMUM_NUMBER {
        println!("{}", n);
    }

    // MAXIMUM_NUMBER = 21; // this statement cause an error
}