// This is needed if other functions are called
// use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bravo_function_1_test() {
        assert_eq!(5678, bravo_function_1());
    }
}

pub fn bravo_function_1() -> i32 {
    println!("This is bravo function");
    5678
}
