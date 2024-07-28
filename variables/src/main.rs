const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // let mut x = 5;
    // println!("The value of x is {x}");
    // x = 6;
    // println!("The value of x is {x}!");
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours");


    let x = 5;
    let x = x +1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x in the outer scope is {x}.");
}
