use super::super::AlfaStruct;

#[cfg(test)]
mod tests {
    use super::AlfaStruct;

    #[test]
    fn alfa_method_1_test() {
        let alfa_test_instance_1 = AlfaStruct {
            ..Default::default()
        };
        assert_eq!(None, alfa_test_instance_1.alfa_field_1)
    }
}

impl AlfaStruct {
    pub fn alfa_method_1(&self) -> i32 {
        println!("This is alfa_method_1");
        self.alfa_method_2();
        1234
    }
}
