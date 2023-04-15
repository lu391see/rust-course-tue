fn main() {
    let mut x: i32 = 0;
    for _i in (1..10) {
        println!("{x}");
        x = x + 1;
    }
    println!("final result: {}", x);
}
