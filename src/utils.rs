use serde_yaml;
use std::collections::HashMap;
use std::fs::File;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn parse_yaml_file(file_name: &str) -> HashMap<String, String> {
    let f = match File::open(file_name) {
        Err(msg) => panic!("Couldn't open {}: {}", file_name, msg),
        Ok(file) => file,
    };

    let parsed_file: HashMap<String, String> = match serde_yaml::from_reader(f) {
        Err(msg) => panic!("Error parsing YAML file!"),
        Ok(parsed) => parsed,
    };

    parsed_file
}
