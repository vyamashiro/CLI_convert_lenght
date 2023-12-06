use dialoguer::{Select, theme::ColorfulTheme};

fn main() {
    println!("-----Bem-vindo ao programa CLI!-----");

    // Defina as opções disponíveis
    let options = vec!["Option 1", "Option 2", "Option 3"];

    // Crie um prompt de seleção
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option:")
        .items(&options)
        .default(0) // Default option
        .interact()
        .unwrap();

    // Execute a lógica com base na Option selecionada
    match selection {
        0 => println!("Você escolheu a Option 1"),
        1 => println!("Você escolheu a Option 2"),
        2 => println!("Você escolheu a Option 3"),
        _ => println!("Option inválida"),
    }
}
