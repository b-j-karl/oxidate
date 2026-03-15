# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

### Added

- `get_num_rows` function to count data rows in a CSV file (excludes header)
- `get_num_cols` function to count columns in a CSV file via header inspection
- `get_csv_schema` function to return column names and inferred types
- `ColumnType` enum (`Integer`, `Float`, `String`) with `Debug` and `PartialEq` derives
- `ColumnSchema` struct to represent a column's name and type
- `infer_column_type` helper to infer a column's type from a sample value
- Test suites for `get_num_rows`, `get_num_cols`, `get_csv_schema`, and `infer_column_type`
- Test data fixture (`test_data/sample.csv`)
