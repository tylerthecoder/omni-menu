pub mod projects_menu {
    use fuzzy_matcher::skim::SkimMatcherV2;
    use fuzzy_matcher::FuzzyMatcher;
    use gtk::prelude::*;
    use gtk::{glib, Application, ApplicationWindow, Entry, ListBox, ScrolledWindow};
    use regex::Regex;
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    const APP_ID: &str = "org.gtk_rs.ProjectsMenu";

    fn get_dev_directories() -> Vec<PathBuf> {
        let home = dirs::home_dir().unwrap();
        let mut directories = Vec::new();
        directories.push(home.join("owl"));
        if let Ok(entries) = fs::read_dir(home.join("dev")) {
            directories.extend(entries.filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_dir() {
                    Some(path)
                } else {
                    None
                }
            }));
        }
        directories
    }

    fn normalize_name(name: &str) -> String {
        let with_spaces = name.replace(['-', '_'], " ");
        let re = Regex::new(r"([a-z])([A-Z])").unwrap();
        let separated = re.replace_all(&with_spaces, "$1 $2");
        separated
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>()
                            + &chars.collect::<String>().to_lowercase()
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn launch_workspace(workspace_name: &str, directory: &PathBuf) {
        Command::new("i3-msg")
            .args(["workspace", workspace_name])
            .spawn()
            .expect("Failed to switch workspace");
        Command::new("i3-msg")
            .args(["layout", "tabbed"])
            .spawn()
            .expect("Failed to set tabbed layout");
        Command::new("terminator")
            .arg("--working-directory")
            .arg(directory)
            .spawn()
            .expect("Failed to launch terminal");
        Command::new("cursor")
            .arg(directory)
            .spawn()
            .expect("Failed to launch editor");
        Command::new("chromium")
            .spawn()
            .expect("Failed to launch browser");
    }

    fn build_ui(app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Project Launcher") // Changed title
            .default_width(400)
            .default_height(600)
            .decorated(true)
            .resizable(true)
            .modal(true)
            .build();
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let search_entry = Entry::new();
        let list_box = ListBox::new();
        list_box.set_vexpand(true);
        list_box.set_selection_mode(gtk::SelectionMode::Single);
        list_box.set_activate_on_single_click(true);
        let scrolled_window = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .child(&list_box)
            .build();
        vbox.append(&search_entry);
        vbox.append(&scrolled_window);
        window.set_child(Some(&vbox));
        let directories = get_dev_directories();
        for dir in &directories {
            let name = dir.file_name().unwrap().to_str().unwrap();
            let normalized = normalize_name(name);
            let label = gtk::Label::new(Some(&normalized));
            label.set_xalign(0.0);
            label.set_margin_start(5);
            label.set_margin_end(5);
            let row = gtk::ListBoxRow::new();
            row.set_child(Some(&label));
            list_box.append(&row);
        }
        let list_box_weak = list_box.downgrade();
        search_entry.connect_activate(move |_entry| {
            let list_box = list_box_weak.upgrade().unwrap();
            if let Some(first_row) = list_box.first_child() {
                let row_count = list_box.observe_children().n_items();
                if row_count == 1 {
                    list_box.emit_by_name::<()>(
                        "row-activated",
                        &[&first_row.downcast_ref::<gtk::ListBoxRow>().unwrap()],
                    );
                } else if row_count > 1 {
                    list_box.select_row(Some(first_row.downcast_ref::<gtk::ListBoxRow>().unwrap()));
                    list_box.grab_focus();
                }
            }
        });
        let list_box_weak = list_box.downgrade();
        let directories_clone = directories.clone();
        search_entry.connect_changed(move |entry| {
            let list_box = list_box_weak.upgrade().unwrap();
            let matcher = SkimMatcherV2::default();
            let query = entry.text();
            while let Some(row) = list_box.first_child() {
                list_box.remove(&row);
            }
            let mut matches: Vec<_> = directories_clone
                .iter()
                .filter_map(|dir| {
                    let name = dir.file_name().unwrap().to_str().unwrap();
                    matcher
                        .fuzzy_match(name, &query)
                        .map(|score| (score, name, dir))
                })
                .collect();
            matches.sort_by(|a, b| b.0.cmp(&a.0));
            for (_, _, dir) in matches {
                let name = dir.file_name().unwrap().to_str().unwrap();
                let normalized = normalize_name(name);
                let label = gtk::Label::new(Some(&normalized));
                label.set_xalign(0.0);
                label.set_margin_start(5);
                label.set_margin_end(5);
                let row = gtk::ListBoxRow::new();
                row.set_child(Some(&label));
                list_box.append(&row);
            }
        });
        let window_weak = window.downgrade();
        list_box.connect_row_activated(move |_list_box, row| {
            let label = row.child().unwrap().downcast::<gtk::Label>().unwrap();
            let selected_dir = directories
                .iter()
                .find(|dir| {
                    normalize_name(dir.file_name().unwrap().to_str().unwrap())
                        == label.text().as_str()
                })
                .unwrap();
            launch_workspace(&label.text(), selected_dir);
            if let Some(window) = window_weak.upgrade() {
                window.close();
            }
        });
        search_entry.grab_focus();
        window.present();
    }

    pub fn run_app() -> glib::ExitCode {
        let app = Application::builder().application_id(APP_ID).build();
        app.connect_activate(build_ui);
        let no_args: [&str; 0] = [];
        app.run_with_args(&no_args)
    }
}
