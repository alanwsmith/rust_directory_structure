use super::super::ExampleStruct;

#[cfg(test)]
mod tests {
    use super::ExampleStruct;

    #[test]
    fn example_associated_function_test() {
         assert_eq!(5678, ExampleStruct::example_associated_function())
    }
}

impl ExampleStruct {
    pub fn example_associated_function() -> i32 {
        5678
    }
}

