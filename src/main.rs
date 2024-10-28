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
        println!("3. Endre lekse");
        println!("4. Endre lekse status");
        println!("5. Fjern lekse");
        println!("6. Avslutt"); 

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
                let lekser: Vec<Lekse> = modules::readjson::read_json_file(filename.clone());

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
                let filename = "src/data.json".to_string();
                let lekser: Vec<Lekse> = modules::readjson::read_json_file(filename.clone());

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
                
                println!("Skriv inn tall for lekse du vil endre:");
                let mut index_input = String::new();
                io::stdin().read_line(&mut index_input).expect("Kunne ikke lese linje.");
                
                if let Ok(index) = index_input.trim().parse::<usize>() {
                    println!("Nåværende navn på lekse: {}", lekser[index - 1].navn);
                    println!("Skriv inn nytt navn:");
                    let mut new_lekse_input = String::new();
                    io::stdin().read_line(&mut new_lekse_input).expect("Kunne ikke lese linje.");
                    println!("{}", new_lekse_input);
                    modules::readjson::change_lekse(filename, index - 1, new_lekse_input); 
                } else {
                    println!("Ugyldig input, vennligst skriv inn et tall.");
                }
            }
            "4" => {
                let filename = "src/data.json".to_string();
                let lekser: Vec<Lekse> = modules::readjson::read_json_file(filename.clone());

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

                println!("Skriv inn tall for lekse du vil endre status på:");
                let mut index_input = String::new();
                io::stdin().read_line(&mut index_input).expect("Kunne ikke lese linje.");
                
                if let Ok(index) = index_input.trim().parse::<usize>() {
                    modules::readjson::toggle_status(filename, index - 1); 
                } else {
                    println!("Ugyldig input, vennligst skriv inn et tall.");
                }
            }
            "5" => {
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
            "6" => {
                println!("Adjø!");
                break;
            }
            _ => {
                println!("Feil input, prøv igjen.");
            }
        } 
    }
}
