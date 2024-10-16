use std::io;
use modules::readjson::Lekse;
use colored::Colorize;
mod modules;

fn main() {
    println!("Velkommen til lekser!");
    loop {
        println!("Velg et alternativ:");
        println!("1. Legg til ny lekse");
        println!("2. Alle lekser");
        println!("3. Fullfør lekse");
        println!("4. Fjern lekse");
        println!("5. Avslutt"); 

        let mut valg = String::new();
        io::stdin().read_line(&mut valg).expect("Kunne ikke lese linje.");

        match valg.trim() {
            "1" => {
                let new_lekse = modules::readjson::get_user_input();

                let filename = "src/data.json".to_string();
                let mut lekser: Vec<Lekse> = modules::readjson::read_json_file(filename.clone());
                lekser.push(new_lekse);
                modules::readjson::write_json_file(filename, &lekser);
            }
            "2" => {
                let filename = "src/data.json".to_string();
                let lekser: Vec<Lekse> = modules::readjson::read_json_file(filename);

                if lekser.is_empty() {
                    println!("Ingen lekser tilgjengelig.");
                } else {
                    println!("Dine lekser:");
                    for (index, lekse) in lekser.iter().enumerate() {
                        if lekse.status {
                            println!("{}. {}", index + 1, lekse.navn.green());
                        } else {
                            println!("{}. {}", index + 1, lekse.navn.red());
                        }
                    }
                }
            }
            "3" => {

            }
            "4" => {
                let filename = "src/data.json".to_string();
                let lekser: Vec<Lekse> = modules::readjson::read_json_file(filename.clone());
                
                if lekser.is_empty() {
                    println!("Ingen lekser å fjerne.");
                } else {
                    println!("Dine lekser:");
                    for (index, lekse) in lekser.iter().enumerate() {
                        if lekse.status {
                            println!("{}. {}", index + 1, lekse.navn.green());
                        } else {
                            println!("{}. {}", index + 1, lekse.navn.red());
                        }
                    }
                }

                println!("Skriv inn tall på lekse du har fullført:");

                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Kunne ikke lese linje.");

                let index: usize = match index.trim().parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(_) => {
                        println!("Ugyldig input.");
                        continue;
                    }
                };

                modules::readjson::remove_lekse_by_index(filename, index);
            }
            "5" => {
                println!("Adjø!");
                break;
            }
            _ => {
                println!("Feil input, prøv igjen.");
            }
        } 
    }
}
