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

use std::path::Path;

use super::wrappers;
use crate::directory_analyzer::DirectoryAnalyzer;
use cursive::{
    align::Align,
    views::{Dialog, SelectView, TextView, ThemedView},
    Cursive,
};

/// Build a succes dialog. By default it is Done!
pub fn build_success(cursive: &mut Cursive, message: Option<&str>) -> ThemedView<Dialog> {
    let msg = message.unwrap_or("Ok!");
    let mut themed_dialog = wrappers::into_dialog(cursive, "Done", TextView::new(msg));
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

/// Return a SelectView constructed from the folder names in the provided path
pub fn build_directory_select(dir: &str) -> Result<SelectView, String> {
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
