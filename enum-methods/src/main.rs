#![allow(dead_code)]

enum Day {
    Monday, Tuesday, Wednesday, Thursday, Friday,
    Saturday, Sunday
}

impl Day {
    fn is_weekday(&self) -> bool {
        match self{
            &Day::Saturday | &Day::Sunday => return false,
            _=> return true
        }
    }
}

fn main() {
    let d = Day::Saturday;

    println!("Is d a weekday? {}", d.is_weekday());
}
