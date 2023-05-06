use crate::bmi::BodyMassIndex;
use crate::height::Height;
use crate::weight::Weight;
use std::str::FromStr;
use std::fs::File;
use std::io::Write;

mod bmi;
mod height;
mod tests;
mod weight;

#[derive(Debug, PartialEq)]
enum BmiError {
    HeightCannotBeZeroOrSmaller,
    WeightCannotBeZeroOrSmaller,
}
/*
fn check_weight(weight: Weight) -> Result<f64, Error> {
    if weight.0 < 0.0 || weight.0 > 200.0 {
        eprintln!("The given weght is illegal! Needs to be within (0, 200)");
        Err(Error)
    } else {
        Ok(weight.0)
    }
}

fn check_height(height: Height) -> Result<f64, Error> {
    if height.0 < 0.0 || height.0 > 2.0 {
        eprintln!(
            "The given height is illegal! Please give a height in centimeres within (0.0, 2.0)"
        );
        Err(Error)
    } else {
        Ok(height.0)
    }
}

let mut index = String::new();
io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");
let index: usize = index
.trim()
.parse()
.expect("Index entered was not a number"); */

fn calc_bmi(w: Weight, h: Height) -> Result<BodyMassIndex, BmiError> {
    if h.0 <= 0.0 {
        return Err(BmiError::HeightCannotBeZeroOrSmaller);
    }
    if w.0 <= 0.0 {
        return Err(BmiError::WeightCannotBeZeroOrSmaller);
    }
    let bmi = w.0 / (h.0 * h.0);

    Ok(BodyMassIndex::new(bmi))
}

fn read_input_as_f64(prompt: &str) -> f64 {
    let handle = std::io::stdin();
    println!("{}", prompt);

    let mut input = String::new();
    // let _ =std::io::stdout().flush();
    match handle.read_line(&mut input) {
        Ok(_) => f64::from_str(input.trim().trim_end_matches('\n')).unwrap_or_else(|e| {
            println!("There was an error while parsing: {}", e);
            read_input_as_f64("Try again!")
        }),

        Err(e) => {
            println!("Reading the input went wrong: {}", e);
            read_input_as_f64("Try again!")
        }
    }
}

fn main() {
    println!("\n<===== Welcome to my Rusty BMI Calculator =====>\n");

    let weight: Weight = Weight(read_input_as_f64(
        "Please enter your weight in kilogramms [kg]:",
    ));
    println!("Your entered weight: {} kg", weight.0);

    let height: Height = Height(read_input_as_f64("Please enter your height in meters [m]:"));
    println!("Your entered height: {} m", height.0);

    // bmi calculation
    let bmi = calc_bmi(weight, height);

    match bmi {
        Ok(bmi) => {
            println!(
                "Your BMI is {}, which is classified as {:?}",
                bmi.value(),
                bmi.category());
            let mut f = File::create("bmi.txt").unwrap();
            f.write_all(bmi.value().to_string().as_bytes()).unwrap();
        },
        Err(e) => println!("Error while calculating! {:?}", e),
    }

    
    
}
