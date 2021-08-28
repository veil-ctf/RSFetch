use std::fs;

pub fn get() -> String {
    let content = fs::read_to_string("/etc/arch-release").expect("Could not read file");
    return content;
}
