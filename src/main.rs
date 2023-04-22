use std::{fmt::Error, str::FromStr};

struct Height(f64);

struct Weight(f64);

struct BMI {
    value: f64,

}

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

fn calc_bmi(w: Weight, h: Height) -> BMI {
    // let w = check_height(height).unwrap();
    // let h = check_weight(weight).unwrap();
    let bmi = w.0 / (h.0 * h.0);
    BMI { value: bmi, }
}

fn main() {
    let (weight, buf_height) = {
        let handle = std::io::stdin();

        println!("Please enter your weight:");
        let mut buf_weight = String::new();
        // let _ =std::io::stdout().flush();
        match handle.read_line(&mut buf_weight) {
            Ok(_) => (),
            Err(error) => print!("error: {error}"),
        }
        let weight = Weight(f64::from_str(&buf_weight.trim_end_matches('\n')).unwrap());
        println!("Your entered weight: {buf_weight} kg");

        println!("Please enter your height:");
        let mut buf_height = String::new();
        match handle.read_line(&mut buf_height) {
            Ok(_) => (),
            Err(error) => print!("error: {error}"),
        }
        (weight, buf_height)
    };
    let height = Height(f64::from_str(&buf_height.trim_end_matches('\n')).unwrap());
    println!("Your entered height: {buf_height}");

    // weight / height
    let bmi = calc_bmi(weight, height);
    println!("Your BMI is {}", bmi.value);
}
