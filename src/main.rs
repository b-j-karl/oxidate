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

#[cfg(test)]
mod tests {
    use super::*; // Gives the test module access to the functions defined in the parent module
    use std::path::Path;

    mod test_get_num_rows {
        use super::*;

        #[test]
        fn happy_path() {
            let path = Path::new("test_data/sample.csv");
            let result = get_num_rows(path).unwrap();
            assert_eq!(result, 3); // 3 data rows, header is excluded by default
        }

        #[test]
        fn file_not_found() {
            let path = Path::new("test_data/nonexistent.csv");
            let result = get_num_rows(path);
            assert!(result.is_err()); // should return an error, not panic
        }
    }
}
