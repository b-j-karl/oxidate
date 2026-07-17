use std::path::Path;

use oxidate::csv_ops::inspect;

fn main() {
    let csv_path = Path::new("test_data/sample.csv");
    inspect(csv_path).unwrap();
}
