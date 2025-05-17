use gtk::glib;
use std::env;

mod projects_menu;
mod search_menu;

fn main() -> glib::ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "search" => search_menu::search_menu::run_app(),
            "projects" => projects_menu::projects_menu::run_app(),
            _ => {
                eprintln!("Usage: rust-menu [search|projects]");
                glib::ExitCode::FAILURE
            }
        }
    } else {
        // Default to projects menu if no argument is provided
        projects_menu::projects_menu::run_app()
    }
}
