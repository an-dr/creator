mod app;
mod file_operator;

use app::App;

mod ui {
    use std::path::PathBuf;

    use cursive::align::HAlign;
    use cursive::event::EventResult;
    use cursive::traits::*;
    use cursive::views::{Dialog, OnEventView, SelectView, TextView};
    use cursive::Cursive;

    use crate::file_operator;

    fn get_select(dir: &str) -> OnEventView<SelectView> {
        let template_path = PathBuf::from(dir);

        let mut select = SelectView::new()
            // Center the text horizontally
            .h_align(HAlign::Center)
            // Use keyboard to jump to the pressed letters
            .autojump();

        // List the dirs
        let directs;
        match file_operator::list_dirs(template_path) {
            Ok(dirs) => {
                println!("Directories:");
                directs = dirs;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                directs = Vec::new();
            }
        }
        select.add_all_str(directs);

        // Sets the callback for when "Enter" is pressed.
        select.set_on_submit(show_next_window);

        OnEventView::new(select)
    }

    pub fn main() {

        let mut siv: cursive::CursiveRunnable = cursive::default();
        
        
        // Let's add a ResizedView to keep the list at a reasonable size
        // (it can scroll anyway).
        let select = get_select("D:/dev-templates/templates");
        siv.add_layer(
            Dialog::around(select.scrollable().fixed_size((20, 10))).title("Select language"),
        );

        siv.run();
    }

    // Let's put the callback in a separate function to keep it clean,
    // but it's not required.
    fn show_next_window(siv: &mut Cursive, selected_element: &str) {
        // siv.pop_layer();
        let text = format!("{selected_element} is selected");
        siv.add_layer(Dialog::around(TextView::new(text)).button("Quit", |s| go_up(s)));
    }

    fn go_up(s: &mut Cursive) {
        s.pop_layer(); // Remove the current dialog layer.
                       // s.add_layer(TextView::new("Good! You can proceed.")); // Add a TextView layer with a message.
    }
}

fn test_app() {
    // std::env::set_var(App::, "VALUE");
    let cwd = std::env::current_dir().expect("Failed to get current directory");

    let mut path = cwd.clone();
    path.push("tests");
    path.push("assets");
    path.push("test_input");

    println!("Test Path: {:?}", path.display());

    let mut app = App::new();
    app.start();

    println!("\n");

    let (files, dirs) = file_operator::collect_files_and_dirs(path);

    for f in files {
        println!("File: {}", f.display());
    }

    for d in dirs {
        println!("Dir:  {}", d.display());
    }
}

fn main() {
    // test_app();
    ui::main();
}
