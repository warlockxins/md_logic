use std::collections::HashMap;
extern crate serde_json;
use serde_json::Value as JsonValue;

use std::fs;
mod context;
mod expression_parser;
mod logic_table;

use context::get_context_var;

fn main() {
    let contents =
        fs::read_to_string("./samples/table.md").expect("Something went wrong reading the file");

    println!("{}", contents);
    let json_str = r#"
    { "season": "Fall", "guestCount": 8 }
    "#;

    let table = logic_table::parse(&contents);

    // read definitions

    println!("{:?}", table);

    let context: serde_json::Value = serde_json::from_str(json_str).unwrap();

    logic_table::run_table(&table, &context);
}
