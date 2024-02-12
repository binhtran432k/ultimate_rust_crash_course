use f_structs_traits::{bunny_nibbles, Bite, Carrot, Grapes};

fn main() {
    let mut carrot = Carrot::new(100.0);
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes::new(100);
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}
