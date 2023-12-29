fn main() {
    //check cargo.toml for the name seaormactixweb
    let result = seaormactixweb::main();

    if let Some(e) = result.err() {
        eprintln!("Error: {}", e);
    }
}
