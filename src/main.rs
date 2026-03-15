use std::path::Path;

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

/// Supported types for CSV columns.
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
    let mut reader = csv::Reader::from_path(csv_path)?;
    let headers = reader.headers()?.clone(); // Clone the headers to own the data
    let mut column_schemas = Vec::with_capacity(headers.len());

    // Get the first row of the CSV to use for type inference
    let first_row = match reader.records().next() {
        Some(Ok(record)) => Some(record), // got a valid row
        Some(Err(e)) => return Err(e),    // parse error, propagate it
        None => None,                     // no data rows
    };

    // Iterate through headers, using the first row for type inference if available
    for (i, header) in headers.iter().enumerate() {
        let column_type = match &first_row {
            Some(row) => infer_column_type(&row[i]),
            None => ColumnType::String, // default to String when no data rows
        };
        column_schemas.push(ColumnSchema {
            name: header.to_string(),
            column_type,
        });
    }
    Ok(column_schemas)
}

fn infer_column_type(value: &str) -> ColumnType {
    // Check if the string is empty
    if value.is_empty() {
        return ColumnType::String; // Treat empty strings as strings
    }
    // Check if the string is all numeric characters
    if value.chars().all(|c| c.is_ascii_digit()) {
        return ColumnType::Integer;
    }
    // Check if the string is a valid float
    if value.parse::<f64>().is_ok() {
        return ColumnType::Float;
    }
    // Otherwise, treat it as a string
    ColumnType::String
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
    mod test_get_csv_schema {
        use super::*;

        #[test]
        fn happy_path() {
            let path = Path::new("test_data/sample.csv");
            let result = get_csv_schema(path).unwrap();
            assert_eq!(
                result,
                vec![
                    ColumnSchema {
                        name: "name".to_string(),
                        column_type: ColumnType::String
                    },
                    ColumnSchema {
                        name: "age".to_string(),
                        column_type: ColumnType::Integer
                    },
                    ColumnSchema {
                        name: "city".to_string(),
                        column_type: ColumnType::String
                    }
                ]
            );
        }
        #[test]
        fn header_only() {
            let path = Path::new("test_data/header_only.csv");
            let result = get_csv_schema(path).unwrap();
            assert_eq!(
                result,
                vec![
                    ColumnSchema {
                        name: "name".to_string(),
                        column_type: ColumnType::String
                    },
                    ColumnSchema {
                        name: "age".to_string(),
                        column_type: ColumnType::String
                    },
                    ColumnSchema {
                        name: "city".to_string(),
                        column_type: ColumnType::String
                    }
                ]
            );
        }
    }

    mod test_infer_column_type {
        use super::*;

        #[test]
        fn integer_from_whole_number() {
            let result = infer_column_type("42");
            assert!(matches!(result, ColumnType::Integer));
        }

        #[test]
        fn integer_from_zero() {
            let result = infer_column_type("0");
            assert!(matches!(result, ColumnType::Integer));
        }

        #[test]
        fn float_from_decimal() {
            let result = infer_column_type("3.14");
            assert!(matches!(result, ColumnType::Float));
        }

        #[test]
        fn float_from_negative_number() {
            let result = infer_column_type("-2.5");
            assert!(matches!(result, ColumnType::Float));
        }

        #[test]
        fn string_from_text() {
            let result = infer_column_type("hello");
            assert!(matches!(result, ColumnType::String));
        }

        #[test]
        fn string_from_empty() {
            let result = infer_column_type("");
            assert!(matches!(result, ColumnType::String));
        }

        #[test]
        fn string_from_mixed_content() {
            let result = infer_column_type("abc123");
            assert!(matches!(result, ColumnType::String));
        }
    }
}
