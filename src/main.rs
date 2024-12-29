mod japanese;
mod english;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Select language: ");
    println!("1. English");
    println!("2. 日本語");

    let mut language = String::new();
    std::io::stdin().read_line(&mut language).unwrap();
    let language = language.trim();

    match language {
        "1" => Ok(english::menu()),
        "2" => japanese::menu(),
        _ => {
            println!("Invalid selection");
            Ok(())
        }
    }
}
