#![allow(unused_variables)]

trait Bite{
    fn bite(self: &mut Self);
}

#[derive(Debug)]

struct Grapes {
    amount_left: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self){
        self.amount_left -= 1;
    }
}

fn main() {
    
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes{ amount_left: 100};
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for a while: {:?}", carrot);
}

#[derive(Debug)] 

struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

fn bunny_nibbles<T: Bite>(eat: &mut T){
   
    for i in 1..5{
        eat.bite();
    }   
}