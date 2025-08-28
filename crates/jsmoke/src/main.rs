fn main() {
    let turn_green = |text: &str| format!("\x1b[92m{text}\x1b[0m");
    println!("Hello from {}!!!", turn_green("jsmoke crates"));
}
