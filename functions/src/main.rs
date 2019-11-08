fn main() {
    print_numbers_to(20);

//    if is_even(30){
//        println!("It is even!");
//    }
}

fn print_numbers_to(num: u32){
    for n in 1..num{
//        println!("{}", n)
        if is_even(n) {
             println!("{} is even!", n);
        }
        else {
            println!("{} is odd", n);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
