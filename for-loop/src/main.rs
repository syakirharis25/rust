fn main() {

    // number example
    //let number = 30..51;

    //for i in number{
    //    println!("The number is {}", i);

    let animals = vec!["Rabbit", "Dog", "Cat"];

    //iteration, repetition method
    //for a in animals.iter(){
    //    println!("The animal name is {}", a); }

    //enumerate method
    for (index, a) in animals.iter().enumerate(){
        println!("The index is {} and the name is {}", index, a);
    }
}
