fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //Using if-statements in a declaration:
    let my_number: i32 = if 1 == 2 { 420 } else { 32 };
    println!("My number is {my_number}");

    finite_loop(10_000);

    let my_array: [i32; 5] = [1,23,4,5,6];
    print_array(my_array);

    count_down(100,0);
    let temp_fahrenheit: f64 = 95.0;
    let temp_celcius: f64 = convert_fahrenheit_to_celcius(temp_fahrenheit);

    println!("{temp_fahrenheit} fahrenheit is equal to {temp_celcius} celcius.");
    let n = 5;
    println!("The {n}th number in the fibonacci sequence is: {}",nth_fib_number(n));
}

fn infinite_loop() {
    loop {
        println!("Again!");
    }
}

fn finite_loop(num_iter: i32) {
    let mut counter: i32 = 0;
    println!("Starting loop...");
    loop {
        if counter == num_iter {
            break;
        }
        counter += 1;
    }
    println!("Loop is done!");
}

fn loop_with_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn print_array(a:[i32;5]) {
    for element in a {
        println!("{element}");
    }
}

fn count_down(from:u32, to: u32) {
    for number in (to..=from).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!");
}

fn convert_fahrenheit_to_celcius(temp:f64) -> f64 {
    (temp-32.0)*5.0/9.0
}

fn nth_fib_number(n:usize) -> usize {
    let mut fib_array = [0;1000];
    fib_array[1] = 1;
    for index in 2..=n {
        fib_array[index] = fib_array[index-1] + fib_array[index-2];
    }
    return fib_array[n];
}