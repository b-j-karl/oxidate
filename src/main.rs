use std::path::Path;

fn main() {
    let csv_path = Path::new("data.csv");
    let num_rows = get_num_rows(&csv_path).unwrap();
    println!("Number of rows: {}", num_rows);
}

fn get_num_rows(csv_path: &Path) -> Result<i32, csv::Error> {
    // Create a CSV reader from the file path. A Reader needs to be mutable, because
    // reading from it changes its internal state.
    let mut reader = csv::Reader::from_path(csv_path)?;
    Ok(reader.records().count() as i32)
}
