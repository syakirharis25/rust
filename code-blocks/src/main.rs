fn main() {
    let x = 10;

    {
        let y = 5;

        println!("x: {} y: {}", x, y);
    }

    // can't find y
    //println!("x: {} y: {}", x, y);
}
