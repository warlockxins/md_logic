use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

type OrderingOperation = Vec<AllCombined>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Opss {
    #[serde(alias = ">")]
    More(OrderingOperation),
    #[serde(alias = "<")]
    Less(OrderingOperation),
    #[serde(alias = "=")]
    Eq(OrderingOperation),
}

impl Opss {
    fn execute(&self) -> AllCombined {
        // println!("ooooooops {:?}", self);
        return AllCombined::OpList(vec![]);
    }
}

fn execute_combined_list(l: &Vec<AllCombined>) -> Vec<AllCombined> {
    l.iter().map(|l_item| l_item.execute()).collect()
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum AllCombined {
    Ops(Opss),
    OpList(Vec<AllCombined>),
    Primitive(Value),
}

#[derive(Debug)]
enum Operations {
    More,
    Less,
    Eq, // MoreEq,
        // LessEq
}

impl AllCombined {
    fn execute(&self) -> AllCombined {
        match self {
            AllCombined::OpList(l) => {
                let s: Vec<AllCombined> = execute_combined_list(&l);
                AllCombined::OpList(s)
            }
            AllCombined::Ops(o) => {
                let (op, arguments) = match o {
                    Opss::Less(l) => (Operations::Less, l),
                    Opss::More(l) => (Operations::More, l),
                    Opss::Eq(l) => (Operations::Eq, l),
                };
                if arguments.len() < 2 {
                    return AllCombined::Primitive(Value::Bool(false));
                }

                let built_list = execute_combined_list(&arguments);

                let left = &built_list[0];
                let right = &built_list[1];

                let res_bool = match op {
                    Operations::Less => left < right,
                    Operations::More => left > right,
                    Operations::Eq => left == right,
                };

                // println!("Oper {:?} {:?}  {:?} ? {}", left, op, right, res_bool);

                AllCombined::Primitive(Value::Bool(res_bool))
            }
            AllCombined::Primitive(v) => AllCombined::Primitive(v.clone()),
        }
    }
}

// Ordering operations
impl std::cmp::Ord for AllCombined {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (
                AllCombined::Primitive(Value::Number(n1)),
                AllCombined::Primitive(Value::Number(n2)),
            ) => {
                let n1_num = n1.as_f64().unwrap_or(0.0);
                let n2_num = n2.as_f64().unwrap_or(0.0);
                if n1_num > n2_num {
                    return std::cmp::Ordering::Greater;
                }
                if n1_num < n2_num {
                    return std::cmp::Ordering::Less;
                }

                return std::cmp::Ordering::Equal;
            }
            (_, _1) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for AllCombined {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for AllCombined {}
// end Ordering operations

mod tests {
    use super::*;

    #[test]
    fn serializes_more_operator_enum_representation() -> Result<()> {
        let cases = vec![
            (r#" { ">" : [3,10] }"#, false),
            (r#" { "<" : [3,10] }"#, true),
            (r#" { "=" : [10.0,10.0] }"#, true),
        ];

        for (data, expected) in cases {
            let p: AllCombined = serde_json::from_str(data)?;
            let res = p.execute();
            assert_eq!(res, AllCombined::Primitive(Value::Bool(expected)));
        }
        Ok(())
    }
}
