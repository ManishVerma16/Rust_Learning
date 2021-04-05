// Http get request

extern crate reqwest;

// use reqwest;
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