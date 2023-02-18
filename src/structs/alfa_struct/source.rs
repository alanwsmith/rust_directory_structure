#[derive(Debug)]
pub struct AlfaStruct {
    pub alfa_field_1: Option<String>,
    pub alfa_field_2: Option<String>,
}

impl Default for AlfaStruct {
    fn default() -> AlfaStruct {
        AlfaStruct {
            alfa_field_1: None,
            alfa_field_2: None,
        }
    }
}