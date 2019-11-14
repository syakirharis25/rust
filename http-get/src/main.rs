extern crate reqwest;

fn main() {

    let response_text = reqwest::get("http://youtube.local/thisdoesntexist")
        .expect("Couldn't make request")
        .text().expect("Could not read response text!");

    println!("Response Text: {}", response_text);

    // match reqwest::get("http://youtube.local/thisdoesntexist"){
    //     Ok(mut response) => {
    //         // Check if 200 OK
    //         if response.status() == reqwest::StatusCode::Ok {
    //             match response.text(){
    //                 Ok(text) => println!("Response Text: {}", text),
    //                 Err(_) => println!("Could not read response text!")
    //             }
    //         } else {
    //             println!("Response was not 200 OK.");
    //         }
    //     }
    //     Err(_) => println!("Could not make the request!")
    // }
}
