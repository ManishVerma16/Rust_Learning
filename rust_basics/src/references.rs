// References -> a different way to refering same data.

fn main(){
    let mut x = 10;
    let xr = &x; // immutable reference

    
    println!("value of x is {}",x);
    println!("value of xr is {}",xr);

    {
        let ish = &mut x;
        *ish += 1;
    println!("value of ish is {}",ish);

        }

}