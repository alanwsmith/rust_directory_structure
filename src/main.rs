#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use rust_directory_structure::functions::*;
use rust_directory_structure::structs::*;

fn main() {
    let alfa = AlfaStruct::new();
    dbg!(&alfa);
    alfa.alfa_method_1();
    bravo_function_1();
    println!("Method bravo_function_2: {}", bravo_function_2());
    println!("Associated function: {}", AlfaStruct::widget());
}
