pub fn check() -> String {
    if std::path::Path::new("/etc/arch-release").exists() == true {
        return "arch-linux-x86_64".to_string();
    } else {
        return false.to_string();
    }
}