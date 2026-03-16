/// Supported types for CSV columns.
#[derive(Debug, PartialEq)]
pub enum ColumnType {
    // Only supporting basic types for now
    Integer,
    Float,
    String,
}
/// Represents the schema of a CSV column.
///
/// # Fields
///
/// * `name` - The name of the column, as a string.
/// * `column_type` - The inferred type of the column, as a ColumnType enum.
#[derive(Debug, PartialEq)]
pub struct ColumnSchema {
    pub name: String,
    pub column_type: ColumnType,
}
