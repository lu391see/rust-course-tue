use std::str::FromStr;

fn calc_bmi(weight: f64, height: f64) -> f64 {

    return weight / (height*height)
}

fn main() {
    let handle = std::io::stdin();
    
    println!("Please enter your weight:");
    let mut buf_weight = String::new();
    // let _ =std::io::stdout().flush();
    match handle.read_line(&mut buf_weight) {
        Ok(_) => (),
        Err(error) => print!("error: {error}"),
    }
    let weight = f64::from_str(&buf_weight.trim_end_matches('\n')).unwrap();
    println!("Your entered weight: {buf_weight} kg");


    println!("Please enter your height:");
    let mut buf_height = String::new();
    match handle.read_line(&mut buf_height) {
        Ok(_) => (),
        Err(error) => print!("error: {error}"),
    }
    let height = f64::from_str(&buf_height.trim_end_matches('\n')).unwrap();
    println!("Your entered height: {buf_height}");

    // weight / height
    let bmi = calc_bmi(weight, height);
    println!("Your BMI is {bmi}");

}