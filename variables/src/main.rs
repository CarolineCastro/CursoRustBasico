const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
//constants can be used outside of main or a function, so ther are global in scope.

fn main() {
    
    let mut misseles = STARTING_MISSILES; //if not use mut, the variable stills immutable.
    let ready = READY_AMOUNT;

    println!("Firing {} of my {} missiles...", ready, misseles);

    misseles = misseles - ready; // now misseles is a mutable variable, so we can run the println with the value are expected.
    println!("{} missiles left", misseles);
}
