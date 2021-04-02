//

fn main(){
    let numbers: [i32; 5] = [10, 22, 33, 40, 51];  // array_name: [datatype ; length]
    println!("index 1 has value {}", numbers[0]);

    for i in numbers.iter(){  // iterating over an array
        println!("{}", i);

    }

    for i in 0..numbers.len(){  // iterating using index
        println!("index {} has value {}",i, numbers[i]); 
    }

    let array = [5; 10];  // creating an array having 5 upto 10 times -> [data ; no_of_times]
    for i in 0..array.len(){ 
        println!("index {} has value {}",i, array[i]); 
    }

}