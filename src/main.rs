#![windows_subsystem = "windows"]

mod changer;


fn main() {
    let result = changer::run();

    match result {
        Ok(_) => println!("Successfully set wallpaper"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
