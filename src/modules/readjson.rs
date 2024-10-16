use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::io;

#[derive(Serialize, Deserialize)]
pub struct Lekse {
    pub navn: String,
    pub status: bool,
}

pub fn read_json_file<T: for<'de> Deserialize<'de>>(filename: String) -> Vec<T> {
    let path = Path::new(&filename);
    
    if !path.exists() {
        println!("Fant ikke fil, begynner på ny tom liste.");
        return vec![];
    }

    let mut fdata = String::new();
    let mut rfile = File::open(path).expect("Kan ikke åpne fil.");
    rfile.read_to_string(&mut fdata).expect("Kan ikke lese fil.");
    
    serde_json::from_str(&fdata).unwrap_or_else(|_| vec![])
}

pub fn write_json_file<T: Serialize>(filename: String, data: &Vec<T>) {
    let mut file = File::create(filename).expect("Kan ikke lage fil.");
    let json = serde_json::to_string_pretty(&data).expect("Kan ikke serialisere data.");
    file.write_all(json.as_bytes()).expect("Kan ikke skrive data til fil.");

    println!("Data oppdatert!");
}

pub fn get_user_input() -> Lekse {
    println!("Skriv inn lekse:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Kunne ikke lese linje.");

    Lekse {
        navn: input.trim().to_string(),
        status: false,
    }
}

pub fn remove_lekse_by_index(filename: String, index: usize) {
    let mut lekser: Vec<Lekse> = read_json_file(filename.clone());

    if index >= lekser.len() {
        println!("Ugyldig index.");
        return;
    }

    lekser.remove(index);
    write_json_file(filename, &lekser);
}

pub fn toggle_status(filename: String, index: usize) {
    let mut lekser: Vec<Lekse> = read_json_file(filename.clone());

    if index >= lekser.len() {
        println!("Ugyldig index.");
        return;
    }

    lekser[index].status = !lekser[index].status;

    write_json_file(filename,&lekser);


    if lekser[index].status {
        println!("Lekse '{}' er nå merket som fullført.", lekser[index].navn);
    } else {
        println!("Lekse '{}' er nå merket som ikke fullført.", lekser[index].navn);
    }
}