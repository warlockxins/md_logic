use crate::get_context_var;
extern crate serde_json;
use serde_json::Value as JsonValue;

#[derive(Debug)]
pub struct Definition {
    pub inputs: Vec<(String, String)>,
    pub outputs: Vec<(String, String)>,
}

const header_row: usize = 0;
const io_row: usize = 1;
const type_row: usize = 2;
const logic_start_row: usize = 4;

#[derive(Debug)]
pub struct Table {
    pub rows: Vec<Row>,
    pub defs: Definition,
}

#[derive(Debug)]
pub struct Row {
    pub cells: Vec<String>,
}

pub fn parse(contents: &String) -> Table {
    let mut newLine = true;
    let mut tmp: String = String::new();

    let mut table: Table = Table {
        rows: vec![],
        defs: Definition {
            inputs: vec![],
            outputs: vec![],
        },
    };

    for c in contents.chars() {
        if c == '|' {
            if newLine {
                table.rows.push(Row { cells: vec![] });

                newLine = false;
            } else {
                if let Some(row) = table.rows.last_mut() {
                    row.cells.push(tmp.trim().to_string());
                }
            }
            tmp.clear();
            continue;
        }

        if c == '\n' {
            newLine = true;
            continue;
        }

        tmp.push(c);
    }

    for col_index in 0..table.rows[header_row].cells.len() {
        let io_def = &table.rows[io_row].cells[col_index];
        let column_variable = &table.rows[header_row].cells[col_index];
        let type_variable = &table.rows[type_row].cells[col_index];

        if io_def.starts_with("-") && io_def.ends_with("-") {
            table
                .defs
                .inputs
                .push((column_variable.clone(), type_variable.clone()));
        } else if io_def.ends_with("-:") {
            table
                .defs
                .outputs
                .push((column_variable.clone(), type_variable.clone()));
        }
    }

    table
}

pub fn run_table(table: &Table, context: &serde_json::Value) {
    let tempN: f64 = 8.0;

    for row_index in logic_start_row..table.rows.len() {
        for d_index in 0..table.defs.inputs.len() {
            let (var_name, var_type) = &table.defs.inputs[d_index];
            let val = get_context_var(var_name, &context);

            let is_eq: bool = match val {
                JsonValue::String(column_string) => true,
                JsonValue::Number(column_number) => column_number.as_f64() == Some(tempN),
                _ => false,
            };

            println!("-------res {}", is_eq);
        }
    }
}

mod tests {
    use super::*;
    use std::fs;

    fn get_test_table() -> Result<Table, String> {
        let contents = fs::read_to_string("./samples/table.md")
            .expect("Something went wrong reading the TEST file");

        //       println!("{}", contents);

        Ok(parse(&contents))
    }

    #[test]
    fn correct_md_table_size() -> Result<(), String> {
        let table = get_test_table()?;
        assert_eq!(table.defs.inputs.len(), 2);
        assert_eq!(table.defs.outputs.len(), 1);
        assert_eq!(table.rows.len(), 6);

        assert_eq!(table.defs.inputs[0].1, "string".to_owned());
        assert_eq!(table.defs.inputs[1].1, "number".to_owned());
        assert_eq!(table.defs.outputs[0].1, "string".to_owned());

        assert_eq!(table.rows[5].cells[2], "\"Roastbeef\"".to_owned());

        Ok(())
    }
}
