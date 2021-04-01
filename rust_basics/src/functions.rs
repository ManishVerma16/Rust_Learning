fn main(){
    print_number_to(10);
    /*
    if is_even(20){
        println!("this is an even number");
    }
    */
}

fn print_number_to(num: u32){
    for i in 1..num{
        if is_even(i){
            println!("{} is an even number.",i);
        }
        else{
            println!("{} is an odd number.",i);
        }
    }
}

/*
fn print_number_to(num:u32){
    for i in 1..num{
        println!("{}",i);
    }
}
*/

fn is_even(num: u32) -> bool { // is_even accepts a number and return boolean values
    return num%2 == 0;
}