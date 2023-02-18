#[derive(Debug)]
pub struct ExampleStruct {
    pub example_field: Option<String>,
}

impl Default for ExampleStruct {
    fn default() -> ExampleStruct {
        ExampleStruct {
            example_field: None,
        }
    }
}