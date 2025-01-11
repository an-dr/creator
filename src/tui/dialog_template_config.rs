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

use super::{builders, strings, wrappers};
use crate::{creator::Creator, directory_analyzer::DirectoryAnalyzer, environment};
use cursive::{
    view::{Nameable, Resizable, Scrollable},
    views::{DummyView, EditView, LinearLayout, TextView},
    Cursive,
};
use std::{collections::HashMap, path::Path};

fn add_variables(layout: &mut LinearLayout, variable_names: &Vec<String>) -> Result<(), String> {
    match variable_names.is_empty() {
        true => Err("No variables".to_string()),
        false => {
            for var in variable_names {
                // Add a TextView and an EditView for each variable
                layout.add_child(TextView::new(format!("{var}:")));
                layout.add_child(EditView::new().with_name(var.clone()));
            }
            Ok(())
        }
    }
}

/// Scans the variable in the folder and asks for the user unput
pub fn show(cursive: &mut Cursive, template_full_path: String) {
    // Get variables
    let d_analyzer = DirectoryAnalyzer::new(&template_full_path);
    let variable_names = d_analyzer.scan_variables();

    // Create a layout with elements
    let mut layout = LinearLayout::vertical();
    let destination = environment::get_current_working_directory();
    layout.add_child(TextView::new(format!("Destination: {destination}")));
    layout.add_child(DummyView::new().fixed_height(1));
    if let Err(e) = add_variables(&mut layout, &variable_names) {
        layout.add_child(TextView::new(e));
    }

    // Create a dialog with buttons around elements
    let mut themed_dialog = wrappers::into_dialog(cursive, strings::CONFIG_DIALOG_TITLE, layout.scrollable());
    let dialog = themed_dialog.get_inner_mut();
    dialog.add_button("Create", move |cursive| {
        create_from_template_and_show_success(cursive, &template_full_path, &destination, &variable_names);
    });
    dialog.add_button("Back", move |cursive| {
        cursive.pop_layer();
    });

    // Add a layer
    cursive.add_layer(themed_dialog);
}

fn create_from_template_and_show_success(cursive: &mut Cursive, srs: &str, dest: &str, var_names: &Vec<String>) {
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
    let dialog = builders::build_success(cursive, Some(&results));
    cursive.add_layer(dialog);
}
