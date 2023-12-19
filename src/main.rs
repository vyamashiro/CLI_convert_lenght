mod handle_selection_menu;
use handle_selection_menu::handle_selection_menu;

mod convert_and_print_result;
use convert_and_print_result::convert_and_print_result;

mod convert_px_to_em;
use convert_px_to_em::convert_px_to_em;

mod convert_px_to_rem;
use convert_px_to_rem::convert_px_to_rem;

mod convert_px_to_percentage;
use convert_px_to_percentage::convert_px_to_percentage;

fn main() {
    println!("{:-^40}", "Length Converter");
    
    match handle_selection_menu() {
        0 => convert_and_print_result(convert_px_to_em, "em"),
        1 => convert_and_print_result(convert_px_to_rem, "rem"),
        2 => convert_and_print_result(convert_px_to_percentage, "%"),
        _ => println!("Invalid Option"),
    }

}
