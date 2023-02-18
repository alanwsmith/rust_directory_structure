use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bravo_function_2_test() {
        assert_eq!(5678, bravo_function_2());
    }
}

pub fn bravo_function_2() -> i32 {
    bravo_function_1()
}
