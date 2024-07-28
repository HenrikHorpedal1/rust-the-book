fn main() {
    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 3);
    let tup_1 = tup.1; //or tup(1)
    println!("First value of the tuple is {tup_1}.");

    //array
    let array: [i32;4] = [1,2,3,4];
    println!("Second value of array: {}:", array[1]);

}
