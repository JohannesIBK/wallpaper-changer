#![windows_subsystem = "windows"]

mod changer;


fn main() {
    match updater::update_application() {
        Ok(_) => println!("Updated application"),
        Err(e) => println!("Error updating application: {e}"),
    } ;
    let result = changer::run();

    match result {
        Ok(_) => println!("Successfully set wallpaper"),
        Err(e) => eprintln!("Error: {e}"),
    }
}


