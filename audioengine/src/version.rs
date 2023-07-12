pub fn print_full_version() {
    println!("DAR audioengine version 0.0.1 Initial");
}

pub fn get_version() -> (i32, i32, i32) {
    let major = 0;
    let minor = 0;
    let patch = 1;
    return (major, minor, patch);
}