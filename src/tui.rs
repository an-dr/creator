// *************************************************************************
//
// Copyright (c) 2025 Andrei Gramakov. All rights reserved.
//
// This file is licensed under the terms of the MIT license.
// For a copy, see: https://opensource.org/licenses/MIT
//
// site:    https://agramakov.me
// e-mail:  mail@agramakov.me
//
// *************************************************************************

use crate::creator::Creator;
use crate::directory_analyzer::DirectoryAnalyzer;
use crate::environment;
use cursive::align::Align;
use cursive::theme::{BaseColor, BorderStyle, Palette};
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, OnEventView, TextView, ThemedView};
use cursive::Cursive;
use std::collections::{HashMap, HashSet};
use std::path::Path;

mod build_view {
    use crate::directory_analyzer::DirectoryAnalyzer;
    use cursive::{
        align::Align,
        event::Key,
        theme::{BaseColor, BorderStyle},
        utils::markup::StyledString,
        view::Resizable,
        views::{Dialog, DummyView, LinearLayout, OnEventView, SelectView, TextView, ThemedView},
        Cursive, View,
    };
    use std::path::Path;

    /// Internal method for building a dialog with a unified keypress logic
    pub fn into_screen_with_keys(title: &str, view: impl View) -> OnEventView<Dialog> {
        let dialog = Dialog::around(view).title(title);
        OnEventView::new(dialog).on_event(Key::Esc, |cursive_inst| {
            if cursive_inst.screen_mut().len() > 1 {
                cursive_inst.pop_layer();
            } else {
                cursive_inst.quit()
            }
        }) // Return OnEventView
    }

    /// Return a SelectView constructed from the folder names in the provided path
    pub fn directory_select(dir: &str) -> Result<SelectView, String> {
        let select = SelectView::new()
            // Center the text horizontally
            .align(Align::center())
            // Use keyboard to jump to the pressed letters
            .autojump();

        // If the dir does not exist write a message
        if !Path::new(dir).exists() {
            return Err(format!("Directory {dir} does not exist"));
        }

        // List the dirs
        let templ_dir = DirectoryAnalyzer::new(dir);
        let (_, directs) = templ_dir.get_items();

        // If no dirs found write a message
        if directs.is_empty() {
            return Err(format!("Directory {dir} is empty").to_string());
        }

        let mut str_paths: Vec<String> = Vec::new();
        for d in directs {
            let s = d.file_name().unwrap().to_str().unwrap().to_string();
            str_paths.push(s);
        }
        Ok(select.with_all_str(str_paths))
    }

    pub fn into_dialog(cursive: &mut Cursive, title: &str, view: impl View) -> ThemedView<Dialog> {
        // Build title
        let mut sstr = StyledString::new();
        sstr.append_styled(title, BaseColor::Cyan.light());
        let mut text = TextView::new("").align(Align::center());
        text.set_content(sstr);

        let spacer = DummyView::new().max_height(1);

        // Construct a window
        let layout = LinearLayout::vertical().child(text).child(spacer).child(view);
        let dialog = Dialog::around(layout);

        // Add border
        let mut theme = cursive.current_theme().clone();
        theme.borders = BorderStyle::Simple;

        ThemedView::new(theme, dialog)
    }

    /// Show a succes dialog. By default it is Done!
    pub fn success(cursive: &mut Cursive, message: Option<&str>) -> ThemedView<Dialog> {
        let msg = message.unwrap_or("Ok!");
        let mut themed_dialog = into_dialog(cursive, "Done", TextView::new(msg));
        let dialog = themed_dialog.get_inner_mut();

        // let mut dialog = Dialog::around(TextView::new(msg));
        dialog.add_button("Exit", move |cursive| {
            cursive.quit();
        });
        dialog.add_button("Back", move |cursive| {
            cursive.pop_layer(); // Hide self
            cursive.pop_layer(); // Hide vars
            cursive.pop_layer(); // Hide group
        });
        themed_dialog
    }
}

const SELECT_GROUP_MSG: &str = "SELECT TEMPLATE GROUP";
const SELECT_ITEM_MSG: &str = "SELECT TEMPLATE";
const VARIABLE_DIALOG_TITLE: &str = "ENTER THE VARIABLE VALUES";

/// Run the tui application
pub fn run() {
    let mut tui = cursive::CursiveRunnable::default();
    tui.add_global_callback('q', |s| s.quit());
    show_main_screen(&mut tui);
    set_theme(&mut tui);
    tui.run();
}

fn set_theme(siv: &mut Cursive) {
    siv.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: BorderStyle::None,
        palette: Palette::retro().with(|palette| {
            use cursive::style::BaseColor::*;

            {
                // First, override some colors from the base palette.
                use cursive::style::Color::TerminalDefault;
                use cursive::style::PaletteColor::*;

                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.light();
                palette[TitlePrimary] = Cyan.dark();
                palette[Secondary] = Cyan.light();
                palette[Highlight] = Cyan.dark();
            }

            {
                // Then override some styles.
                use cursive::style::Effect::*;
                use cursive::style::PaletteStyle::*;
                use cursive::style::Style;
                palette[HighlightInactive] = Style::from(White.dark()).combine(Bold);
                palette[EditableTextCursor] = Style::secondary().combine(Reverse).combine(Bold)
            }
        }),
    });
}

/// Shows Template type selection dialog
fn show_main_screen(cursive: &mut Cursive) {
    let template_storage_path = environment::get_storage_path();
    let result = build_view::directory_select(&template_storage_path);

    let dialog = match result {
        Ok(mut select) => {
            // Configure opening of the next dialog
            select.set_on_submit(move |cursive_inst: &mut Cursive, selected: &str| {
                let group_path = format!("{template_storage_path}/{selected}");
                show_template_select(cursive_inst, group_path);
            });
            let select_wrapped = select.align(Align::center()).scrollable().full_screen();
            build_main_screen(SELECT_GROUP_MSG, select_wrapped)
        }
        Err(e) => build_main_screen(
            SELECT_GROUP_MSG,
            TextView::new(e.to_string()).align(Align::center()).full_screen(),
        ),
    };
    cursive.add_layer(dialog);
}

/// Shows template selection dialog
fn show_template_select(cursive: &mut Cursive, group_full_path: String) {
    // Create select
    let result = build_view::directory_select(&group_full_path);

    let dialog = match result {
        Ok(mut select) => {
            // Configure the next window
            select.set_on_submit(move |cursive_inst, selected_template: &str| {
                let template_full_path = format!("{group_full_path}/{selected_template}");
                show_variable_input_dialog(cursive_inst, template_full_path);
            });
            // Dialog
            build_screen(SELECT_ITEM_MSG, select.scrollable().full_screen())
        }
        Err(e) => build_main_screen(
            SELECT_GROUP_MSG,
            TextView::new(e.to_string()).align(Align::center()).full_screen(),
        ),
    };

    cursive.add_layer(dialog);
}

/// Scans the variable in the folder and asks for the user unput
fn show_variable_input_dialog(cursive: &mut Cursive, template_full_path: String) {
    let destination = environment::get_current_working_directory();
    let d_analyzer = DirectoryAnalyzer::new(&template_full_path);
    let variable_names = d_analyzer.scan_variables();

    let mut dialog_w_el = match variable_names.is_empty() {
        false => {
            let mut layout = LinearLayout::vertical();
            let mut sorted_vars: Vec<_> = variable_names.clone().into_iter().collect();
            sorted_vars.sort();
            for var in &sorted_vars {
                // Add a TextView and an EditView for each variable
                layout.add_child(TextView::new(format!("{var}:")));
                layout.add_child(EditView::new().with_name(var.clone()));
            }
            build_screen(VARIABLE_DIALOG_TITLE, layout.scrollable())
        }
        true => build_screen(VARIABLE_DIALOG_TITLE, TextView::new("No variables")),
    };

    let dialog = dialog_w_el.get_inner_mut();
    dialog.add_button("Create", move |cursive| {
        create_from_template_and_show_success(cursive, &template_full_path, &destination, &variable_names);
    });
    dialog.add_button("Back", move |cursive| {
        cursive.pop_layer();
    });

    // Add border
    let mut theme = cursive.current_theme().clone();
    theme.borders = BorderStyle::Simple;

    cursive.add_layer(ThemedView::new(theme, dialog_w_el));
}

// Show a succes dialog. By default it is Done!
fn show_success(cursive: &mut Cursive, message: Option<&str>) {
    let success_view = build_view::success(cursive, message);
    cursive.add_layer(success_view);
}

fn create_from_template_and_show_success(cursive: &mut Cursive, srs: &str, dest: &str, var_names: &HashSet<String>) {
    // Collect input values print
    let mut input_values: HashMap<String, String> = HashMap::new();
    for var in var_names {
        let value = cursive
            .call_on_name(var, |view: &mut EditView| view.get_content())
            .unwrap_or_default();
        // if not empty ot not whitespaces
        if !value.trim().is_empty() {
            input_values.insert(var.clone(), value.to_string());
        }
    }
    let mut creator = Creator::new(Path::new(srs), Path::new(dest));
    creator.set_var_values(&input_values);
    creator.create().expect("Failed to create from template");

    let mut results = String::new();
    let src = creator.get_source().to_str().unwrap();
    let dsc = creator.get_destination().to_str().unwrap();
    results.push_str(&format!("Source: {src}\n"));
    results.push_str(&format!("Destination: {dsc}\n"));

    for (k, v) in creator.get_var_values() {
        results.push_str(&format!("{k}: {v}\n"));
    }
    // Show results in a new dialog
    show_success(cursive, Some(&results));
}

fn build_main_screen(title: &str, view: impl View) -> OnEventView<Dialog> {
    let mut layout = LinearLayout::vertical();
    layout.add_child(DummyView::new().max_height(1)); // spacer
    layout.add_child(view);

    // Add app name with version
    let mut sstr = StyledString::new();
    sstr.append_styled(
        format!("Creator v{}", env!("CARGO_PKG_VERSION")),
        BaseColor::Black.light(),
    );
    let mut text = TextView::new("").align(Align::center());
    text.set_content(sstr);
    layout.add_child(text);

    build_view::into_screen_with_keys(title, layout) // Return OnEventView
}

// Returns a dialog with the provided title and view
fn build_screen(title: &str, view: impl View) -> OnEventView<Dialog> {
    let mut layout = LinearLayout::vertical();
    layout.add_child(DummyView::new().max_height(1)); // spacer
    layout.add_child(view);
    build_view::into_screen_with_keys(title, layout) // Return OnEventView
}
