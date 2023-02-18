use super::super::AlfaStruct;

#[cfg(test)]
mod tests {
    use super::AlfaStruct;

    #[test]
    fn alfa_function_1_test() {
         assert_eq!(251, AlfaStruct::widget())
    }
}

impl AlfaStruct {
    pub fn widget() -> i32 {
        251
    }
}

