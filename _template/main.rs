#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use source_code_doc_maker::functions::*;
use source_code_doc_maker::structs::*;

fn main() {
    let example = ExampleStruct::new();
    println!("{}", example.example_method());
}
