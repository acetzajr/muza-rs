fn main() {
    match muza::run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}
