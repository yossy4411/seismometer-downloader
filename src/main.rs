mod japanese;
mod english;
mod gui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gui_result = gui::loader::run();
    if gui_result.is_ok() {
        return Ok(());
    }
    println!("Sorry, GUI failed to load. Now running CLI.");
    println!("Caution: CLI is not fully implemented yet.");
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
