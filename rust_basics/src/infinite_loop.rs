fn main(){
    let mut n = 0;
    // loop with finite condition
    loop{
        n += 1;

        if n==7{
            continue;
        }

        if n>10{
            break;
        }
        println!("This value of n is {}", n);
    }
}


/* infinite loop
loop{
        n += 1;
        println!("This value of n is {}", n);
    }

*/