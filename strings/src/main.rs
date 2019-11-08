fn main() {
    let my_string = String::from("How's it going? My name is Dom.");

    // Length
    println!("Length: {}", my_string.len());

    // Is Empty?
    println!("String is empty? {}", my_string.is_empty());

    for token in my_string.split_whitespace(){
        println!("{}", token);
    }

    println!("Does the string contain 'Dom'? {}", my_string.contains("Dom"));

    println!("Welcome to your tutorial on Strings!");

    println!("{}", my_string);
}
