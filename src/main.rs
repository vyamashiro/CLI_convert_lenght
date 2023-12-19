use dialoguer::{Select, theme::ColorfulTheme};
use std::io;
use std::num::ParseFloatError;

mod convert_px_to_em;
use convert_px_to_em::convert_px_to_em;

mod convert_px_to_rem;
use convert_px_to_rem::convert_px_to_rem;

mod convert_px_to_percentage;
use convert_px_to_percentage::convert_px_to_percentage;

fn main() {
    println!("{:-^40}", "Length Converter");

    // Defina as opções disponíveis
    let options = vec!["Convert px to em", "Convert px to rem", "Convert px to %"];

    // Crie um prompt de seleção
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option:")
        .items(&options)
        .default(0) // Default option
        .interact()
        .unwrap();
    
    pub fn get_typed_value() -> Result<f32, ParseFloatError> {
        let mut input = String::new();
        println!("Type the number in px to be converted to em");
    
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading console");
    
        input.trim().parse()

    }

    fn convert_and_print_result(convert_fn: fn(f32) -> f32, unit: &str) {
        match get_typed_value() {
            Ok(numero) => {
                println!("O resultado é {}{}", convert_fn(numero), unit);
            }
            Err(_e) => {
                println!("Error, try again. Type a valid a number please");
            }
        }
    }
    
    match selection {
        0 => convert_and_print_result(convert_px_to_em, "em"),
        1 => convert_and_print_result(convert_px_to_rem, "rem"),
        2 => convert_and_print_result(convert_px_to_percentage, "%"),
        _ => println!("Invalid Option"),
    }

}
