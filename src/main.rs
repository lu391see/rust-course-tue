use std::str::FromStr;

mod tests;

struct Height(f64);

struct Weight(f64);

#[derive(Debug, PartialEq)]
struct BodyMassIndex {
    value: f64,
    category: BmiCategory,
}

#[derive(Debug, PartialEq)]
enum BmiCategory {
    // SevereUnderweight,
    ModerateUnderweight,
    // MildUnderweight,
    NormalRange,
    Overweight,
    ObeseClass1,
    // ObeseClass2,
    // ObeseClass3,
}

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
        return Err(BmiError::HeightCannotBeZeroOrSmaller)
    }
    if w.0 <= 0.0 {
        return Err(BmiError::WeightCannotBeZeroOrSmaller)
    }
    let bmi = w.0 / (h.0 * h.0);

    let range = match bmi {
        x if x < 18.5 => BmiCategory::ModerateUnderweight,
        x if x < 25.0 => BmiCategory::NormalRange,
        x if x < 30.0 => BmiCategory::Overweight,
        _ => BmiCategory::ObeseClass1,
    };

    Ok(BodyMassIndex {
        value: bmi,
        category: range,
    })
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
        Ok(bmi) => println!(
            "Your BMI is {}, which is classified as {:?}",
            bmi.value, bmi.category
        ),
        Err(e) => println!("Error while calculating! {:?}", e)
    }
}
