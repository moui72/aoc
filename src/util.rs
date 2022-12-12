use std::fs;

pub fn load_input_file(path: &str) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file")
}
