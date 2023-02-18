use super::super::ExampleStruct;

#[cfg(test)]
mod tests {
    use super::ExampleStruct;

    #[test]
    fn alfa_method_1_test() {
        let example_method_test_instance = ExampleStruct {
            ..Default::default()
        };
        assert_eq!(1234, example_method_test_instance.example_method())
    }
}

impl ExampleStruct {
    pub fn example_method(&self) -> i32 {
        1234
    }
}
