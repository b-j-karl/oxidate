use std::path::Path;

use crate::schema::{ColumnSchema, ColumnType};

/// Returns the number of rows in a CSV file, excluding the header row.
///
/// # Arguments
/// * `csv_path` - A reference to a Path object representing the file path of the CSV file.
pub fn get_num_rows(csv_path: &Path) -> Result<usize, csv::Error> {
    // Create a CSV reader from the file path. A Reader needs to be mutable, because
    // reading from it changes its internal state.
    let mut reader = csv::Reader::from_path(csv_path)?;
    Ok(reader.records().count())
}

/// Returns the number of columns in a CSV file, based on the header row.
///
/// # Arguments
/// * `csv_path` - A reference to a Path object representing the file path of the CSV file.
pub fn get_num_cols(csv_path: &Path) -> Result<usize, csv::Error> {
    let mut reader = csv::Reader::from_path(csv_path)?;
    // .headers() returns a reference to a StringRecord of the headers. A StringRecord
    // is basically a vector if strings representing a row of the CSV.
    Ok(reader.headers()?.len())
}

/// Returns the schema of a CSV file, which includes the column names and their inferred types.
///
/// # Arguments
/// * `csv_path` - A reference to a Path object representing the file path of the CSV file.
pub fn get_csv_schema(csv_path: &Path) -> Result<Vec<ColumnSchema>, csv::Error> {
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

/// Read the first n rows of a CSV file and return them as a vector of StringRecords.
///
/// # Arguments
/// * `csv_path` - A reference to a Path object representing the file path of the CSV file.
/// * `n` - The number of rows to read from the CSV file.
///
/// # Returns
/// * `Ok(Some(StringRecord))` - If the CSV file has at least n rows, returns the first
///    n rows as a vector of Strings. If the CSV file has fewer than n rows, returns
///    all available rows.
fn csv_head(csv_path: &Path, n: usize) -> Result<Vec<Vec<String>>, csv::Error> {
    let mut reader = csv::Reader::from_path(csv_path)?;

    // Fix n the number of rows in CSV if number of rows is less than n
    let csv_length = get_num_rows(csv_path)?;
    let n = n.min(csv_length);

    // We need to clone here to avoid holding a mutable and immutable reference on the reader at the same time.
    let headers = reader.headers()?.clone();
    let mut head = Vec::with_capacity(n + 1);

    head.push(headers.iter().map(|s| s.to_string()).collect()); // Add headers as the first row
    for result in reader.records().take(n) {
        let record = result?; // Propagate any errors while reading records

        // Convert the StringRecords to Vec<String> and push to rows
        head.push(record.iter().map(|s| s.to_string()).collect());
    }
    Ok(head)
}

#[cfg(test)]
mod tests {
    use super::*;
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
    mod test_csv_head {
        use super::*;

        #[test]
        fn happy_path() {
            let path = Path::new("test_data/sample.csv");
            let result = csv_head(path, 2).unwrap();
            assert_eq!(
                result,
                vec![
                    vec!["name".to_string(), "age".to_string(), "city".to_string()], // headers
                    vec![
                        "Anahera".to_string(),
                        "30".to_string(),
                        "Auckland".to_string()
                    ],
                    vec![
                        "Ben".to_string(),
                        "25".to_string(),
                        "Wellington".to_string()
                    ]
                ]
            );
        }
        #[test]
        fn file_not_found() {
            let path = Path::new("test_data/nonexistent.csv");
            let result = csv_head(path, 2);
            assert!(result.is_err()); // should return an error, not panic
        }
        #[test]
        fn zero_rows() {
            let path = Path::new("test_data/sample.csv");
            let result = csv_head(path, 0).unwrap();
            assert_eq!(
                result,
                vec![vec![
                    "name".to_string(),
                    "age".to_string(),
                    "city".to_string()
                ]]
            )
        }
        #[test]
        fn n_gt_csv_length() {
            let path = Path::new("test_data/sample.csv");
            let result = csv_head(path, 10).unwrap();

            // There should just be 4 vectors inside the main outer vector
            assert_eq!(result.len(), 4); // 3 data rows + 1 header row
        }
    }
}
