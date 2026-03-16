use std::path::Path;

use oxidate::csv_ops::{get_csv_schema, get_num_cols, get_num_rows};

fn main() {
    let csv_path = Path::new("test_data/sample.csv");
    let num_rows = get_num_rows(csv_path).unwrap();
    let num_cols = get_num_cols(csv_path).unwrap();
    let schema = get_csv_schema(csv_path).unwrap();
    println!("Number of rows: {}", num_rows);
    println!("Number of columns: {}", num_cols);
    for column in schema {
        println!("Column: {}, Type: {:?}", column.name, column.column_type);
    }
}
