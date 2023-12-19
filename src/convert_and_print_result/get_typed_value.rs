use std::io;
use std::num::ParseFloatError;

/// function to get the value typed by the user
pub fn get_typed_value() -> Result<f32, ParseFloatError> {
    let mut input = String::new();
    println!("Type the number in px to be converted to em");

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading console");

    input.trim().parse()
}