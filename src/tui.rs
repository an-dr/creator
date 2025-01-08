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
use cursive::align::{Align, HAlign};
use cursive::event::Key;
use cursive::theme::BaseColor;
use cursive::utils::markup::StyledString;
use cursive::view::IntoBoxedView;
use cursive::views::{Dialog, EditView, LinearLayout, OnEventView, SelectView, TextView};
use cursive::Cursive;
use cursive::{traits::*, CursiveRunnable};
use std::io;
use std::alloc::Layout;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Duration;

const SELECT_GROUP_MSG: &str = "SELECT TEMPLATE GROUP";
const SELECT_ITEM_MSG: &str = "SELECT TEMPLATE";
const DIALOG_TITLE: &str = "ENTER THE VARIABLE VALUES";

const SIZE_SELECT: (u16, u16) = (30, 10);

/// Run the tui application
pub fn run() {
    let mut tui = cursive::CursiveRunnable::default();
    tui.add_global_callback('q', |s| s.quit());
    show_main_screen(&mut tui);
    set_theme(&mut tui);
    tui.run();
}

fn set_theme(siv: &mut CursiveRunnable) {
    siv.load_toml(include_str!("tui_theme.toml")).unwrap();
}

/// Shows Template type selection dialog
fn show_main_screen(cursive: &mut CursiveRunnable) {
    let template_storage_path = environment::get_storage_path();
    let mut select = build_select_view(&template_storage_path).unwrap();

    select.set_on_submit(move |cursive_inst: &mut Cursive, selected: &str| {
        let group_path = format!("{template_storage_path}/{selected}");
        show_template_select(cursive_inst, group_path);
    });

    let dialog = build_main_dialog(SELECT_GROUP_MSG, select.scrollable().full_screen());
    cursive.add_layer(dialog);
}

/// Shows template selection dialog
fn show_template_select(cursive: &mut Cursive, group_full_path: String) {
    // Create select
    let mut select = build_select_view(&group_full_path).unwrap();
    select.set_on_submit(move |cursive_inst, selected_template: &str| {
        let template_full_path = format!("{group_full_path}/{selected_template}");
        show_variable_input_form(cursive_inst, template_full_path);
    });

    // Build the dialog
    let sel_events = OnEventView::new(select)
        .on_event(Key::Esc, |cursive_inst| {
            cursive_inst.pop_layer();
        })
        .scrollable()
        .fixed_size(SIZE_SELECT);

    let dialog = build_dialog(SELECT_ITEM_MSG, sel_events);
    cursive.add_layer(dialog);
}

/// Scanns the variable in the folder and asks for the user unput
fn show_variable_input_form(cursive: &mut Cursive, template_full_path: String) {
    let destination = environment::get_current_working_directory();
    let d_analyzer = DirectoryAnalyzer::new(&template_full_path);
    let variable_names = d_analyzer.scan_variables();

    // Create a vertical layout to hold input fields
    let mut layout = LinearLayout::vertical();
    let mut sorted_vars: Vec<_> = variable_names.clone().into_iter().collect();
    sorted_vars.sort();
    for var in &sorted_vars {
        // Add a TextView and an EditView for each variable
        layout.add_child(TextView::new(format!("{var}:")));
        layout.add_child(EditView::new().with_name(var.clone()));
    }

    // Get a dialog and add buttons to it
    let mut dialog_w_events = build_dialog(DIALOG_TITLE, layout.scrollable());
    let dialog = dialog_w_events.get_inner_mut();
    dialog.add_button("Back", move |cursive| {
        cursive.pop_layer();
    });
    dialog.add_button("Create", move |cursive| {
        create_from_template(cursive, &template_full_path, &destination, &variable_names);
    });
    cursive.add_layer(dialog_w_events);
}

fn create_from_template(
    cursive: &mut Cursive,
    srs: &str,
    dest: &str,
    var_names: &HashSet<String>,
) {
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
    cursive.add_layer(Dialog::info(results));
    // exit_after(cursive, 3);
}

fn build_main_dialog(title: &str, view: impl View) -> OnEventView<Dialog> {
    let mut layout = LinearLayout::vertical();
    layout.add_child(view);

    // Add message
    let mut sstr = StyledString::new();
    sstr.append_styled(
        format!("Creator v{}", env!("CARGO_PKG_VERSION")),
        BaseColor::Magenta.dark(),
    );
    let mut text = TextView::new("").align(Align::center());
    text.set_content(sstr);
    layout.add_child(text);

    let dialog = Dialog::around(layout).title(title);
    OnEventView::new(dialog).on_event(Key::Esc, |cursive_inst| {
        if cursive_inst.screen_mut().len() > 1 {
            cursive_inst.pop_layer();
        } else {
            cursive_inst.quit()
        }
    }) // Return OnEventView
}

// Returns a dialog with the provided title and view
fn build_dialog(title: &str, view: impl View) -> OnEventView<Dialog> {
    let mut layout = LinearLayout::vertical();
    layout.add_child(view);

    let dialog = Dialog::around(layout).title(title);
    OnEventView::new(dialog).on_event(Key::Esc, |cursive_inst| {
        if cursive_inst.screen_mut().len() > 1 {
            cursive_inst.pop_layer();
        } else {
            cursive_inst.quit()
        }
    }) // Return OnEventView
}

/// Return a SelectView constructed from the folder names in the provided path
fn build_select_view(dir: &str) -> Result<SelectView, String> {
    let select = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center)
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

/// The function for the future use that will exit the application after a certain time
fn _exit_after(siv: &mut Cursive, sec: u64) {
    let duration = Duration::from_secs(sec);
    let quit_callback = siv.cb_sink().clone();

    std::thread::spawn(move || {
        std::thread::sleep(duration);
        quit_callback
            .send(Box::new(|s: &mut Cursive| s.quit()))
            .unwrap();
    });
}
