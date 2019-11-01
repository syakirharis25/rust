const MAXIMUM_NUMBER: u8 = 20;

fn main() {
    for n in 1..MAXIMUM_NUMBER {
        println!("{}", n);
    }

    // will produce error
    //MAXIMUM_NUMBER = 30;
}
