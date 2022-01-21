extern crate reqwest;

fn main() {
    let response_text = reqwest::get("http://localhost:8000/")
        .expect("Failed to send request")
        .text()
        .expect("Failed to read response");

    println!("{}", response_text);

    match reqwest::get("http://localhost:8000/") {
        Ok(mut response) => {
            // check if 200 Ok
            if response.status() == reqwest::StatusCode::Ok {
                match response.text() {
                    Ok(text) => println!("{}", text),
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Error: {}", response.status());
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
