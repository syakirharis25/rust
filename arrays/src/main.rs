fn main() {

    // array size
    //let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    let numbers = [2; 400];

    // for loop iter() method
    // for n in numbers.iter(){

    // for loop len() method
    for i in 0..numbers.len(){
        println!("{}", numbers[i]);
    }
}
