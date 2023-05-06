use crate::bmi::BodyMassIndex;
use crate::height::Height;
use crate::weight::Weight;

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

fn read_input_as_f64(prompt: &str, metric: &str) -> f64 {
    use inquire::CustomType;

    let amount = CustomType::<f64>::new(prompt)
        .with_formatter(&|i| format!("{:.2} {}", i, metric))
        .with_error_message("Please type a valid number")
        .with_help_message(&format!(
            "Check the right formatting with si metric: [{}]. Use decimal point as a separator",
            metric
        ))
        .prompt();

    match amount {
        Ok(x) => {
            log::debug!("Entered value: {}", x);
            x
        }
        Err(e) => {
            log::error!("Couldn't process input: {}", e);
            read_input_as_f64("Try again!", metric)
        }
    }
}

fn main() {
    println!("\n<===== Welcome to my Rusty BMI Calculator =====>\n");

    let weight: Weight = Weight(read_input_as_f64(
        "Please enter your weight in kilogramms [kg]:",
        "kg",
    ));
    log::debug!("Your entered weight: {} kg", weight.0);

    let height: Height = Height(read_input_as_f64(
        "Please enter your height in meters [m]:",
        "m",
    ));
    log::debug!("Your entered height: {} m", height.0);

    let bmi = calc_bmi(weight, height);

    match bmi {
        Ok(bmi) => println!(
            "Your BMI is {}, which is classified as {:?}",
            bmi.value(),
            bmi.category()
        ),
        Err(e) => println!("Error while calculating! {:?}", e),
    }
}
