use rust_directory_structure::functions::*;
use rust_directory_structure::structs::*;

fn main() {
    let alfa = AlfaStruct {
        alfa_field_1: Some(String::from("Hello, Alfa")),
        ..Default::default()
    };
    dbg!(&alfa);
    alfa.alfa_method_1();
    bravo_function_1();
    println!("Method bravo_function_2: {}", bravo_function_2());
    println!("Associated function: {}", AlfaStruct::widget());
}
