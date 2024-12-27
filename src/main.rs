mod app;
mod file_operator;
use app::App;

mod ui {
    use crate::file_operator;
    use cursive::align::HAlign;
    use cursive::event::Key;
    use cursive::traits::*;
    use cursive::views::{Dialog, OnEventView, SelectView, TextView};
    use cursive::Cursive;
    use std::path::PathBuf;
    use std::time::Duration;

    fn get_select_view(dir: &str) -> SelectView {
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
                // println!("Directories:");
                directs = dirs;
            }
            Err(e) => {
                show_get_select_view_error(&e.to_string());
                directs = Vec::new();
            }
        }
        select.add_all_str(directs);
        select
    }

    fn show_get_select_view_error(msg: &str) {
        eprintln!("Error: {}", msg);
    }

    fn show_select_lang(_cursive: &mut cursive::Cursive, title: &str, path: &str) {
        let mut select = get_select_view(path);
        select.set_on_submit(show_template_select);

        let sel_events = OnEventView::new(select)
            .on_event(Key::Esc, |cursive| cursive.quit())
            .scrollable()
            .fixed_size((20, 10));
        _cursive.add_layer(Dialog::around(sel_events).title(title));
    }

    // Let's put the callback in a separate function to keep it clean,
    // but it's not required.
    fn show_template_select(siv: &mut Cursive, selected_element: &str) {
        let path = format!("{}/{}", "D:/dev-templates/templates", selected_element);
        let mut select = get_select_view(&path);
        select.set_on_submit(cb_selected_template);

        let sel_events = OnEventView::new(select)
            .on_event(Key::Esc, |siv| {
                siv.pop_layer();
            })
            .scrollable()
            .fixed_size((20, 10));

        let dialog = Dialog::around(sel_events).title("Select Template");
        siv.add_layer(dialog);
    }

    fn exit_after(siv: &mut Cursive, sec: u64) {
        let duration = Duration::from_secs(sec);
        let quit_callback = siv.cb_sink().clone();

        std::thread::spawn(move || {
            std::thread::sleep(duration);
            quit_callback
                .send(Box::new(|s: &mut Cursive| s.quit()))
                .unwrap();
        });
    }

    fn cb_selected_template(siv: &mut Cursive, selected_element: &str) {
        let text_view = TextView::new(format!("Created item based on {}", selected_element));
        let dialog = Dialog::around(text_view).button("Perfect!", |s| s.quit()); // Exits the application when clicked

        siv.add_layer(dialog);
    }

    /// Main application function staruing TUI
    pub fn main() {
        let mut siv: cursive::CursiveRunnable = cursive::default();
        show_select_lang(&mut siv, "Select Language", "D:/dev-templates/templates");

        siv.add_global_callback('q', |s| s.quit());
        siv.run();
    }
}

fn test_app() {
    std::env::set_var(App::TEMPLATES_PATH_VAR_NAME, "VALUE");
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
    ui::main();
}
