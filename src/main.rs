use std::path::Path;

use oxidate::csv_ops::{csv_head, get_csv_schema, get_num_cols, get_num_rows};

fn main() {
    let csv_path = Path::new("test_data/sample.csv");
    let num_rows = get_num_rows(csv_path).unwrap();
    let num_cols = get_num_cols(csv_path).unwrap();
    let schema = get_csv_schema(csv_path).unwrap();
    let head = csv_head(csv_path, 5).unwrap();
    println!("Number of rows: {}", num_rows);
    println!("Number of columns: {}", num_cols);
    for column in schema {
        println!("Column: {}, Type: {:?}", column.name, column.column_type);
    }
    println!();
    let sep = ", ";
    println!("    {}", head[0].join(sep));
    for (i, row) in head[1..].iter().enumerate() {
        println!("[{}] {}", i, row.join(sep));
    }
}
