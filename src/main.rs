use std::path::Path;

fn main() {
    let csv_path = Path::new("test_data/sample.csv");
    let num_rows = get_num_rows(csv_path).unwrap();
    let num_cols = get_num_cols(csv_path).unwrap();
    println!("Number of rows: {}", num_rows);
    println!("Number of columns: {}", num_cols);
}

fn get_num_rows(csv_path: &Path) -> Result<usize, csv::Error> {
    // Create a CSV reader from the file path. A Reader needs to be mutable, because
    // reading from it changes its internal state.
    let mut reader = csv::Reader::from_path(csv_path)?;
    Ok(reader.records().count())
}
fn get_num_cols(csv_path: &Path) -> Result<usize, csv::Error> {
    let mut reader = csv::Reader::from_path(csv_path)?;
    // .headers() returns a reference to a StringRecord of the headers. A StringRecord
    // is basically a vector if strings representing a row of the CSV.
    Ok(reader.headers()?.len())
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
            assert_eq!(result, 3_usize); // 3 data rows, header is excluded by default
        }

        #[test]
        fn file_not_found() {
            let path = Path::new("test_data/nonexistent.csv");
            let result = get_num_rows(path);
            assert!(result.is_err()); // should return an error, not panic
        }
    }
    mod test_get_num_cols {
        use super::*;

        #[test]
        fn happy_path() {
            let path = Path::new("test_data/sample.csv");
            let result = get_num_cols(path).unwrap();
            assert_eq!(result, 3_usize); // 3 columns: name, age, city
        }
        #[test]
        fn file_not_found() {
            let path = Path::new("test_data/nonexistent.csv");
            let result = get_num_cols(path);
            assert!(result.is_err()); // should return an error, not panic
        }
    }
}
