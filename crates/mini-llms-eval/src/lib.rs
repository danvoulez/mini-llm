pub fn structured_output_rate(schema_valid: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        schema_valid as f64 / total as f64
    }
}
