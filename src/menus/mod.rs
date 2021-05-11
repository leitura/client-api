use regex::Regex;
use colored::*;
use std::io::stdin;

pub struct NumberStatus {
    pub msg: &'static str,
    pub number: String,
    pub status: bool
}


pub fn show_menu() -> u32 {
    let options = vec![
        format!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue()),
        format!("{}", "O Mentalista".bright_red()),
        format!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue()),
        format!("{}", "Raj, escolhe uma carta".green()),
        format!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue()),
        format!("{}) -> {}", "1".bright_blue(), "Iniciar".bright_green()),
        format!("{}) -> {}", "2".bright_blue(), "Atualizar tokens".bright_green()),
        format!("{}) -> {}", "3".bright_blue(), "Inserir \"token personalizado\"".bright_green()),
        format!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue()),
        format!("{}", "qual você mais quer no momento ".green())
    ];

    print!("\x1B[2J\x1B[1;1H");

    for option in options.into_iter() {
        println!("{}", option);
    }

    let mut choice = String::new();
    stdin().read_line(&mut choice)
        .expect("põe número direito");
    let choiced: u32 = choice.replace("\n", "").parse::<u32>().expect("Número inválido!");

    choiced
}


pub fn get_telephone() -> NumberStatus {
    let re = Regex::new(r"(\d{2})(\d{8,9})").unwrap();
    println!("{}", "Põe 55 + DDD + número".bright_green());
    let mut number = String::new();

    stdin().read_line(&mut number)
        .expect(&"põe número direito".red());

    if re.is_match(&number) {
        NumberStatus{
            msg: "Número válido!",
            number: number.replace("\n",""),
            status: true
        }
    } else {
        NumberStatus{
            msg: "Número inválido!",
            number: "40028922".to_owned(),
            status: false
        }
    }

}
