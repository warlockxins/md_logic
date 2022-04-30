#[derive(Debug)]
pub struct Table {
    pub rows: Vec<Row>,
}

#[derive(Debug)]
pub struct Row {
    pub cells: Vec<String>,
}

pub fn parse(contents: &String) -> Table {
    let mut newLine = true;
    let mut tmp: String = String::new();

    let mut table: Table = Table { rows: vec![] };

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

    table
}
