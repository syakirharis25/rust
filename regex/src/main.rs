extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(\w{5})").unwrap();
    let text = "dcode";

    // show found its match or not by boolean true or false
    //println!("Found match? {}", re.is_match(text));

    match re.captures(text) {

        // first method
        // Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()),

        // second method
        Some(caps) => println!("Found match: {}", &caps[0]),
        None => println!("Could not find match...")
    }
}
