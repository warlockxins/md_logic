extern crate serde_json;

use std::fs;
mod context;
mod expression_parser;
mod logic_table;

mod json_logic;

fn main() {
    // todo - wrap in rest apis
    let contents =
        fs::read_to_string("./samples/table.md").expect("Something went wrong reading the file");

    let json_str = r#"
    { "season": "Fall", "guestCount": 8 }
    "#;

    let table = logic_table::parse(&contents);
    let context: serde_json::Value = serde_json::from_str(json_str).unwrap();

    let result = logic_table::run_table(&table, &context);
    match result {
        Ok(response) => {
            println!("success {:?}", response)
        }
        Err(message) => {
            println!("failed: {}", message)
        }
    }
}
