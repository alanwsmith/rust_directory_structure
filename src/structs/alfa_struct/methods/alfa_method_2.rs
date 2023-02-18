use super::super::AlfaStruct;

#[cfg(test)]
mod tests {
    use super::AlfaStruct;

    #[test]
    fn alfa_method_2_test() {
        let alfa_test_instance_2 = AlfaStruct {
            ..Default::default()
        };
        assert_eq!(1234, alfa_test_instance_2.alfa_method_1())
    }
}

impl AlfaStruct {
    pub fn alfa_method_2(&self) {
        println!("This is alfa_method_2 called inside alfa_method_1");
    }
}
