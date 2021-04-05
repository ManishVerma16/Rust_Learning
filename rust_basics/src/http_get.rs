// Http get request

// extern crate reqwest;

use reqwest;

fn main(){
    let response_text = reqwest::get("http://www.google.com")
    .expect("Couldn't make request")
    .text().expect("Could not read response text!");

    println!("Response Text: {}", response_text);
}


/*
fn main() {
    match reqwest::get("http://www.google.com"){
        Ok(mut response) => {
            // check if 200 found
            if response.status == reqwest::StatusCode::Ok {
                match response.text()
                {
                    Ok(text) => println!("Response Text: {}", text),
                Err(_) => println!("Could not read response text!"),
            }
            }
            else {
                println!("Response not found!");
            }

        }
        Err(_) => println!("Could not make request!"),
    }
}

*/