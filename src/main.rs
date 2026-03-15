use std::path::Path;

fn main() {
    let csv_path = Path::new("test_data/sample.csv");
    let num_rows = get_num_rows(csv_path).unwrap();
    let num_cols = get_num_cols(csv_path).unwrap();
    println!("Number of rows: {}", num_rows);
    println!("Number of columns: {}", num_cols);
}

/// Supported types for CSV columns.
enum ColumnType {
    // Only supporting basic types for now
    Integer,
    Float,
    String,
}
/// Represents the schema of a CSV column.
///
/// # Arguments
///
/// * `name` - The name of the column, as a string.
/// * `column_type` - The inferred type of the column, as a ColumnType enum.
struct ColumnSchema {
    name: String,
    column_type: ColumnType,
}

/// Returns the number of rows in a CSV file, excluding the header row.
///
/// # Arguments
/// * `csv_path` - A reference to a Path object representing the file path of the CSV file.
fn get_num_rows(csv_path: &Path) -> Result<usize, csv::Error> {
    // Create a CSV reader from the file path. A Reader needs to be mutable, because
    // reading from it changes its internal state.
    let mut reader = csv::Reader::from_path(csv_path)?;
    Ok(reader.records().count())
}

/// Returns the number of columns in a CSV file, based on the header row.
///
/// # Arguments
/// * `csv_path` - A reference to a Path object representing the file path of the CSV file.
fn get_num_cols(csv_path: &Path) -> Result<usize, csv::Error> {
    let mut reader = csv::Reader::from_path(csv_path)?;
    // .headers() returns a reference to a StringRecord of the headers. A StringRecord
    // is basically a vector if strings representing a row of the CSV.
    Ok(reader.headers()?.len())
}

/// Returns the schema of a CSV file, which includes the column names and their inferred types.
///
/// # Arguments
/// * `csv_path` - A reference to a Path object representing the file path of the CSV file.
fn get_csv_schema(csv_path: &Path) -> Result<Vec<ColumnSchema>, csv::Error> {
    Ok(vec![])
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
    mod test_get_headers {
        use super::*;

        #[test]
        fn happy_path() {
            let path = Path::new("test_data/sample.csv");
            let result = get_csv_schema(path).unwrap();
            assert_eq!(
                result,
                vec!["name".to_string(), "age".to_string(), "city".to_string()]
            );
        }
    }
}
