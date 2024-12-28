use crate::creator_operations;
use crate::directory::Directory;
use crate::creator::Creator;
use cursive::align::HAlign;
use cursive::event::Key;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, OnEventView, SelectView, TextView};
use cursive::Cursive;
use std::path::PathBuf;
use std::time::Duration;

pub struct Tui {
    ui: cursive::CursiveRunnable,
}

impl Tui {
    const SELECT_GROUP_MSG: &str = "Select template group";
    const SELECT_ITEM_MSG: &str = "Select template";

    /// Tui constructor
    pub fn new() -> Tui {
        let tui = Tui {
            ui: cursive::default(),
        };
        tui
    }

    /// Run the tui application
    pub fn run(&mut self) {
        self.ui.add_global_callback('q', |s| s.quit());
        self.show_select_group(creator_operations::get_storage_path());
        self.ui.run();
    }

    /// Shows Template type selection dialog
    fn show_select_group(&mut self, template_storage_path: String) {
        let mut select = Tui::build_select_view(&template_storage_path);

        select.set_on_submit(move |cursive_inst: &mut Cursive, selected: &str| {
            let storage = template_storage_path.clone();
            Tui::cb_show_template_select(cursive_inst, selected, storage);
        });

        let sel_events = OnEventView::new(select)
            .on_event(Key::Esc, |cursive| cursive.quit())
            .scrollable()
            .fixed_size((20, 10));
        self.ui
            .add_layer(Dialog::around(sel_events).title(Tui::SELECT_GROUP_MSG));
    }

    /// Shows template selection dialog
    fn cb_show_template_select(
        cursive: &mut Cursive,
        selected_group: &str,
        template_storage_path: String,
    ) {
        // Create select
        let group_full_path = format!("{}/{}", template_storage_path, selected_group);
        let mut select = Tui::build_select_view(&group_full_path);
        select.set_on_submit(move |s, v| {
            let group_path = group_full_path.clone();
            Tui::cb_show_template_form(s, v, group_path);
        });

        // Build the dialog
        let sel_events = OnEventView::new(select)
            .on_event(Key::Esc, |cursive_inst| {
                cursive_inst.pop_layer();
            })
            .scrollable()
            .fixed_size((20, 10));
        let dialog = Dialog::around(sel_events).title(Tui::SELECT_ITEM_MSG);
        cursive.add_layer(dialog);
    }

    fn cb_show_template_form(
        cursive: &mut Cursive,
        template_name: &str,
        template_group_path: String,
    ) {
        let template_full_path = format!("{}/{}", template_group_path, template_name);
        
        let variable_names = creator_operations::get_variables(&template_full_path);

        // Create a vertical layout to hold input fields
        let mut layout = LinearLayout::vertical();

        for var in &variable_names {
            // Add a TextView and an EditView for each variable
            layout.add_child(TextView::new(format!("{}:", var)));
            layout.add_child(EditView::new().with_name(var.clone()));
        }

        // Wrap the layout in a Dialog with a submit button
        let dialog_title = format!("Enter Details \n Template: {}", template_full_path);
        let dialog = Dialog::around(layout.scrollable())
            .title(dialog_title)
            .button("Cancel", move |cursive| {
                cursive.pop_layer();
            })
            .button("Submit", move |cursive| {
                Tui::cb_copy_template(
                    cursive,
                    &template_full_path,
                    &creator_operations::get_current_working_directory(),
                    variable_names.clone(),
                );
            });

        let dialog_ev = OnEventView::new(dialog).on_event(Key::Esc, move |cursive| {
            cursive.pop_layer();
        });
        cursive.add_layer(dialog_ev);
    }

    fn cb_copy_template(
        cursive: &mut Cursive,
        source: &str,
        destination: &str,
        variable_names: Vec<String>,
    ) {
        let mut results = String::new();

        results.push_str(&format!("Source: {}\n", source));
        results.push_str(&format!("Destination: {}\n", destination));
        // Collect input values
        for var in &variable_names {
            let value = cursive
                .call_on_name(var, |view: &mut EditView| view.get_content())
                .unwrap_or_default();
            results.push_str(&format!("{}: {}\n", var, value));
        }
        // Show results in a new dialog
        cursive.add_layer(Dialog::info(results));
    }

    /// Return a SelectView constructed from the folder names in the provided path
    fn build_select_view(dir: &str) -> SelectView {
        let template_path = PathBuf::from(dir);

        let mut select = SelectView::new()
            // Center the text horizontally
            .h_align(HAlign::Center)
            // Use keyboard to jump to the pressed letters
            .autojump();

        // List the dirs
        let templ_dir = Directory::new(template_path.to_str().unwrap());
        let (_, directs) = templ_dir.get_items();
        
        let mut str_paths: Vec<String> = Vec::new();
        for d in directs{
            let s = d.file_name().unwrap().to_str().unwrap().to_string();
            str_paths.push(s);
        }
        select.add_all_str(str_paths);
        select
    }
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
