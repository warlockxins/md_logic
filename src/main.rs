extern crate serde_json;

use std::time::SystemTime;

use serde_json::Value;

use crate::json_logic::AllCombined;
mod context;
mod expression_parser;
mod logic_table;

mod json_logic;

fn main() {
    const DATA: &str = r#" 
    {
      "and": [
            { ">": [3, 1] },
            { "<": [1, 3] }
            ]
          }
          "#;
    // { "=" : [{"var" : "challenger.name"}, "Dread Pirate Roberts"] }

    let context: Value = serde_json::from_str(
        r#"{
            "rounds" : 4, 
            "champ" : {
              "name" : "Fezzig",
              "height" : 223
            },
            "challenger" : {
              "name" : "Dread Pirate Roberts",
              "height" : 183
            }
          }"#,
    )
    .unwrap();

    let now = SystemTime::now();

    let p: AllCombined = serde_json::from_str(DATA).unwrap();

    p.execute(&context);

    let elapsed = now.elapsed();

    match elapsed {
        Ok(duration) => println!("elapsed {}", duration.as_millis()),
        Err(error) => println!("Error: {error:?}"),
    }
}
