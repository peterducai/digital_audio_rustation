pub fn print_full_version() {
    let result = get_version();
    println!("DAR engine {}.{}.{} codename: {}", result.0, result.1, result.2, result.3);
}

pub fn get_version() -> (i32, i32, i32, String) {
    let major = 0;
    let minor = 0;
    let patch = 1;
    let name: &'static str = "Initial";
    return (major, minor, patch, name.to_string());
}