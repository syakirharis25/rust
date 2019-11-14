fn main() {
    // let name = String::from("Domenic");
    //
    // println!("Character at index 1: {}", match name.chars().nth(1) {
    //     Some(c) => c.to_string(),
    //     None => "No character at index 8!".to_string()
    // });
    println!("Occupation is {}", match get_occupation("Freddy"){
        Some(o) => o,
        None => "No occupation found!"
    });
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Domenic" => Some("Software Developer"),
        "Michael" => Some("Dentist"),
        _=> None
    }
}
