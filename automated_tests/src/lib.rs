pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn add_two(number: i32) -> i32 {
    add(number,2)
}


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn this_will_panic() {
        println!("Sui")
    }
}
