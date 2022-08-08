use crate::context::var_to_operand;
use crate::expression_parser::executor::interpret;
use crate::expression_parser::operand::{Operand, Operator};
use crate::expression_parser::tokenizer::Tokenizer;
use serde_json::Value;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Definition {
    pub inputs: Vec<(String, String)>,
    pub outputs: Vec<(String, String)>,
}

const HEADER_ROW: usize = 0;
const IO_ROW: usize = 1;
const TYPE_ROW: usize = 2;
const LOGIC_START_ROW: usize = 4;

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
    let mut new_line = true;
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
            if new_line {
                table.rows.push(Row { cells: vec![] });

                new_line = false;
            } else {
                if let Some(row) = table.rows.last_mut() {
                    row.cells.push(tmp.trim().to_string());
                }
            }
            tmp.clear();
            continue;
        }

        if c == '\n' {
            new_line = true;
            continue;
        }

        tmp.push(c);
    }

    for col_index in 0..table.rows[HEADER_ROW].cells.len() {
        let io_def = &table.rows[IO_ROW].cells[col_index];
        let column_variable = &table.rows[HEADER_ROW].cells[col_index];
        let type_variable = &table.rows[TYPE_ROW].cells[col_index];

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

pub fn run_table(
    table: &Table,
    context: &serde_json::Value,
) -> Result<Vec<HashMap<String, Operand>>, String> {
    let mut outputs: Vec<HashMap<String, Operand>> = vec![];
    let mut row_is_true;

    for row_index in LOGIC_START_ROW..table.rows.len() {
        row_is_true = true;

        for col_index in 0..table.defs.inputs.len() {
            let (var_name, _var_type) = &table.defs.inputs[col_index];
            let input_operand = var_to_operand(var_name, &context);
            let column_value = &table.rows[row_index].cells[col_index];
            let mut parser = Tokenizer::new(&column_value);

            parser.parse()?;

            let start_with_operand = parser.starts_with_operand();
            if !start_with_operand {
                parser.insert_start(input_operand);
                parser.insert_start(Operand::OperatorToken(Operator::E));
            } else {
                parser.insert_start(input_operand);
            }

            let expression = parser.to_postfix()?;
            let expr_result = interpret(&expression);
            if let Some(
                // Operand::Boolean(true)
                Operand::Primitive(Value::Bool(true)),
            ) = expr_result.get(0)
            {
                row_is_true = true;
            } else {
                row_is_true = false;
                break;
            }
        }

        if row_is_true {
            // println!("should set output to {:?}", table.defs.outputs);
            let mut output_result: HashMap<String, Operand> = HashMap::new();

            let offset = table.defs.inputs.len();
            for col_index in 0..table.defs.outputs.len() {
                let column_output_value = &table.rows[row_index].cells[col_index + offset];
                let (out_key, _operand_type) = &table.defs.outputs[col_index];
                output_result.insert(
                    out_key.to_owned(),
                    Operand::Primitive(Value::String(column_output_value.to_owned())),
                );
            }

            outputs.push(output_result);
        }
    }

    Ok(outputs)
}

mod tests {
    use super::*;
    use std::fs;

    fn get_test_table() -> Result<Table, String> {
        let contents = fs::read_to_string("./samples/table.md")
            .expect("Something went wrong reading the TEST file");

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

    #[test]
    fn execute_md_table() -> Result<(), String> {
        let table = get_test_table()?;
        let json_str = r#"
        { "season": "Fall", "guestCount": 8 }
        "#;

        let context: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let res = run_table(&table, &context)?;

        assert_eq!(res.len(), 1);

        let firs_res = &res[0];
        assert_eq!(firs_res.contains_key("desiredDish"), true);
        assert_eq!(
            firs_res.get("desiredDish"),
            Some(&Operand::Primitive(Value::String(
                "\"Spaceribs\"".to_owned()
            )))
        );

        Ok(())
    }
}
