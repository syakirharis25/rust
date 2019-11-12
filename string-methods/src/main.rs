fn main() {
    /* Replace */
    {
        let my_string = String::from("Rust is fantastic!");
        println!("After replace: {}", my_string.replace("fantastic", "great"));

    }
    /* Lines */
    {
        let my_string = String::from("The weather is\nnice\noutside mate!");

        for line in my_string.lines(){
            println!("[ {} ]", line);
        }
    }

    /* Split */
    {
        let my_string = String::from("Leave+a+like+if+you+enjoyed!");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{}", my_string);
        println!("At index 2: {}", tokens[2]);
    }

    /* Trim */
    {
        let my_string = String::from("   My name is Domenic   \n\r");

        println!("before trim: {}", my_string);
        println!("after trim: {}", my_string.trim());
    }

    /* Chars */
    {
        let my_string = String::from("dcode on YouTube");
        println!("{}", my_string);

        /* Get character at index */
        match my_string.chars().nth(4) {
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4.")
        }
    }
}
