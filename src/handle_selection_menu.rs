use dialoguer::{Select, theme::ColorfulTheme};

/// function to handle the selection menu
pub fn handle_selection_menu() -> usize {
    let options = vec!["Convert px to em", "Convert px to rem", "Convert px to %"];

    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option:")
        .items(&options)
        .default(0) // Default option
        .interact()
        .unwrap()
}