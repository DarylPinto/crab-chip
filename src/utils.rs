use std::path::Path;

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn trim_file_ext(file_name: &str) -> String {
    match Path::new(file_name).file_stem() {
        Some(pretty_name) => String::from(pretty_name.to_str().unwrap()),
        None => String::from(file_name),
    }
}
