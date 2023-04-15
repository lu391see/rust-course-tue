use std::str::FromStr;

fn program(args: Vec<String>) {
    let x = args
        .iter()
        .nth(1)
        .map(|s: &String| {
            i32::from_str(s)
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

#[test]
fn test_my_program(){
    let test_vec = vec![String::from("foo"), String::from("1")];
    program(test_vec);
}

#[test]
fn test_my_program_42(){
    let test_vec = vec![String::from("foo"), String::from("42")];
    program(test_vec);
}

#[test]
// #[should_panic] to test panic
fn test_my_program_bar(){
    let test_vec = vec![String::from("foo"), String::from("bar")];
    program(test_vec);
}

fn main() {
    let args = std::env::args().collect();
    program(args)
}
