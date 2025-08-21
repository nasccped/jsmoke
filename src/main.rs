//! # `JSmoke` main file

fn main() {
    let turn_green = |x: &str| format!("\x1b[92m{x}\x1b[0m");
    println!("Welcome to {}!", turn_green("Jsmoke"));
}
