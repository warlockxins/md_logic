use std::collections::HashMap;
extern crate serde_json;
use serde_json::Value as JsonValue;

use std::fs;
mod parser;
mod table_reader;

#[derive(Debug)]
struct Definition {
    inputs: Vec<(String, String)>,
    outputs: Vec<(String, String)>,
}

const header_row: usize = 0;
const io_row: usize = 1;
const type_row: usize = 2;
const logic_start_row: usize = 4;

fn main() {
    let contents =
        fs::read_to_string("./samples/table.md").expect("Something went wrong reading the file");

    println!("{}", contents);
    let json_str = r#"
    { "season": "Fall", "guestCount": 8 }
    "#;

    let context: serde_json::Value = serde_json::from_str(json_str).unwrap();
    println!("json is {:?}", context);

    let table = table_reader::parse(&contents);

    // read definitions
    let mut defs = Definition {
        inputs: vec![],
        outputs: vec![],
    };

    for col_index in 0..table.rows[header_row].cells.len() {
        let io_def = &table.rows[io_row].cells[col_index];
        let column_variable = &table.rows[header_row].cells[col_index];
        let type_variable = &table.rows[type_row].cells[col_index];

        if io_def.starts_with("-") && io_def.ends_with("-") {
            defs.inputs
                .push((column_variable.clone(), type_variable.clone()));
        } else if io_def.ends_with("-:") {
            defs.outputs
                .push((column_variable.clone(), type_variable.clone()));
        }
    }

    println!("def {:?}", defs);
    println!("{:?}", table);

    let tempN: f64 = 8.0;

    for row_index in logic_start_row..table.rows.len() {
        for d_index in 0..defs.inputs.len() {
            //            println!("{}", table.rows[row_index].cells[d_index]);
            let (var_name, var_type) = &defs.inputs[d_index];
            let val = get_context_var(var_name, &context);

            // println!(" --- var value {:?}", val);
            let is_eq: bool = match val {
                JsonValue::String(column_string) => true,
                JsonValue::Number(column_number) => column_number.as_f64() == Some(tempN),
                _ => false,
            };

            println!("-------res {}", is_eq);
        }
    }
}

fn get_context_var(name: &String, context: &serde_json::Value) -> serde_json::Value {
    let v: Vec<&str> = name.split('.').collect();

    let mut cur: &serde_json::Value = &context;
    for key in v.iter() {
        cur = match &cur[key] {
            JsonValue::Null => return JsonValue::Null,
            val => &val,
        }
    }

    cur.clone()
}
