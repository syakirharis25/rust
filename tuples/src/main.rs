fn main() {
    // store numbers
    // let tup1 = (20, 25, 30, 35);

    // store multiple type of data
    // let tup1 = (20, "Rust", 3.4, false);

    // tuple within tuple
    // let tup1 = (20, "Rust", 3.4, false, (1, 4, 7));

    let tup1 = (45, 6.7, "Computer");
    let (a, b, c) = tup1;

    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
}
