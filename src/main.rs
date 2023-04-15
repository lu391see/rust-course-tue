use std::str::FromStr;

fn program(args: Vec<String>) {
    let x = args
        .iter()
        .nth(1)
        .map(|s: &String| {
            i32::from_str(&s)
        })
        .unwrap_or(Ok(0))
        .unwrap_or_else(|err| {
            println!("Error while parsing: {}", err);
            0
        });

    for i in x..10 {
        println!("{i}");
    };
}

fn main() {
    let args = std::env::args().collect();
    program(args)
}
