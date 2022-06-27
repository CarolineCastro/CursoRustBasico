const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}

/* This challenge is:
1. Declare all variables on one line, keeping their properties (mutable or not) 
2. Explain the types of each variable on the same line
3. See which warning appears when putting expression missiles - ready inside println.
        ANSWER warning: variable does not need to be mutable 
4. Add another variable but don't use it.
        ANSWER warning: unused variable: `jet`
5. Modify a constant in main()
        ANSWER error[E0070]: invalid left-hand side of assignment
                --> src/main.rs:5:18
                |
                |
                |      READY_AMOUNT = 1;
                |      -------------^
                |      |
                |      cannot assign to this expression
*/
