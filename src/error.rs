

pub fn error(line: i32, message: &String) {
    report(line, &"".to_string(), message)
}

pub fn report(line: i32, location: &String, message: &String) {
    let report: String = format!("[line {line}] Error {location}: {message}");
    println!("{report}");
}