use serde::{Serialize, Deserialize};
use std::{io, process};
use std::error::Error;
use csv::StringRecord;
use regex::Regex;

#[derive(Serialize, Deserialize)]
struct MixShoesData {
    email: String,
    age: String,
    interested: bool,
    know: bool,
    preference: i32,
    cities: Vec<String>,
    marks: Vec<String>,
    colors: Vec<String>,
    sizes: String,
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut listing : Vec<MixShoesData> = Vec::new();

    for result in reader.records() {
        let record = result?;
        clean_data(&record, &mut listing)?;
    }

    std::fs::write(
        "result.json",
        serde_json::to_string_pretty(&listing).unwrap(),
    )?;

    Ok(())
}

fn clean_data(record: &StringRecord, listing: &mut Vec<MixShoesData>) -> Result<(), Box<dyn Error>> {
    let email = String::from(&record[1]);
    let age = sanitize_age(&record[2]);
    let interested =  sanitize_interested(&record[3]);
    let know =  sanitize_know(&record[4]);
    let preference =  sanitize_preference(&record[5]).unwrap_or_else(|_| 2);
    let cities =  sanitize_cities(&record[6]);
    let marks = sanitize_marks(&record[7]);
    let colors = sanitize_colors(&record[8]);
    let sizes = String::from(&record[9]);

    let mix_shoes_data = MixShoesData { email, age, interested, know, preference, cities, marks, colors, sizes };
    listing.push(mix_shoes_data);
    Ok(())
}

fn sanitize_colors(colors: &str) -> Vec<String> {
    match colors {
        "Peu m'importe" => Vec::with_capacity(0),
        "Toutes les couleurs !" => Vec::with_capacity(0),
        _ => {
            let colors = colors.to_string();
            colors.replace(&['&', ',', '!'][..], " ")
                .split(" ")
                .map(|c| c.trim().replace("__", " ").replace("...", "").replace("_", " "))
                .filter(|c| !c.is_empty())
                .map(|s| String::from(s).to_uppercase())
                .filter(|c| !["CE", "QUI", "FLASH", "ET", "LE", "LA", "LES"][..].contains(&&c[..]))
                .collect()
        }
    }
}

fn sanitize_marks(marks: &str) -> Vec<String> {

    match marks {
        "Je n'ai pas de pr??f??rence" => Vec::with_capacity(0),
        _ => {
            let re_nike = Regex::new(r"(?P<first>AIR|DOC|NIKE)\s*(?P<middle>\w*)\s*(?P<last>1|MARTEN|TN)").unwrap();
            re_nike.replace_all(&String::from(marks).to_uppercase(), "${first}_${middle}_${last}")
                .split(&['/', ',', ' '][..])
                .map(|c| c.trim().replace("__", " ").replace("_", " "))
                .filter(|c| !c.is_empty())
                .filter(|c| !["ET", "LE", "LA", "LES"][..].contains(&&c[..]))
                .map(String::from)
                .collect()
        }
    }
}

fn sanitize_cities(cities_str: &str) -> Vec<String> {
    let re_saint = Regex::new(r"saint ").unwrap();
    let re_en = Regex::new(r" en ").unwrap();
    let cities = re_saint.replace_all(cities_str, "saint-").to_string();
    let cities = re_en.replace_all(&cities, "-en-").to_string();

    cities
        .split(&['/', ',', ' '][..])
        .map(|c| c.trim().to_uppercase())
        .filter(|c| !c.is_empty())
        .filter(|c| !c.contains("ET"))
        .map(String::from)
        .collect()
}

fn sanitize_age(age_str: &str) -> String {
    age_str
        .replace("Entre ", "")
        .replace(" ans", "")
        .replace("et", "-")
        .replace("Plus de ", "+")
}

fn sanitize_interested(interested_str: &str) -> bool {
    match interested_str {
        "Non" => false,
        _ => true
    }
}

fn sanitize_know(know_str: &str) -> bool {
    match know_str {
        "Oui" => true,
        _ => false
    }
}

fn sanitize_preference(preference_str: &str) -> Result<i32, ()> {
    match preference_str {
        "Neuve" => Ok(1),
        "Peu m'importe" => Ok(0),
        "D'occasion" => Ok(-1),
        _ => Err(())
    }
}

