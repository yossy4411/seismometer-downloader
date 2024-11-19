mod japanese;
mod english;

fn main() {
    println!("Select language: ");
    println!("1. English");
    println!("2. 日本語");

    let mut language = String::new();
    std::io::stdin().read_line(&mut language).unwrap();
    let language = language.trim();

    match language {
        "1" => english::menu(),
        "2" => japanese::menu(),
        _ => println!("Invalid selection"),
    }
}
