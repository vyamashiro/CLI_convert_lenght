use dialoguer::{Select, theme::ColorfulTheme};
use std::io;

mod convert_px;
use convert_px::convert_px;

fn main() {
    println!("-----Bem-vindo ao programa CLI!-----");

    // Defina as opções disponíveis
    let options = vec!["Convert px to em", "Convert px to rem", "Convert px to %"];

    // Crie um prompt de seleção
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option:")
        .items(&options)
        .default(0) // Default option
        .interact()
        .unwrap();

    // Execute a lógica com base na Option selecionada
    match selection {
        0 => {

            let mut input = String::new();
            println!("Type the number in px to be converted to em");
    
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading console");

            // let value: Result<i32, _> = input.trim().parse();
            let value: Result<f32, _> = input.trim().parse();

            match value {
                Ok(numero) => {
                    // Chamando a função com o valor convertido
                    println!("O resultado é {} em", convert_px(numero));
        
                    // Agora você pode usar 'numero' como um inteiro
                    // ... faça o que precisar com o inteiro aqui ...
                }
                Err(e) => {
                    println!("Erro ao converter a string para inteiro: {}", e);
                }
            }

            // println!("O resultado e {}", convert_px(value))
        },
        1 => println!("Você escolheu a Option 2"),
        2 => println!("Você escolheu a Option 3"),
        _ => println!("Option inválida"),
    }
}
