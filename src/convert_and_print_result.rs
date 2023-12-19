mod get_typed_value;
use get_typed_value::get_typed_value;

/// function to convert and print the result
pub fn convert_and_print_result(convert_fn: fn(f32) -> f32, unit: &str) {
    match get_typed_value() {
        Ok(numero) => {
            println!("O resultado é {}{}", convert_fn(numero), unit);
        }
        Err(_e) => {
            println!("Error, try again. Type a valid a number please");
        }
    }
}