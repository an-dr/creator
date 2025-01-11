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

use super::{builders, dialog_template_config, strings, wrappers};
use cursive::{
    align::Align,
    view::{Resizable, Scrollable},
    views::TextView,
    Cursive,
};

/// Shows template selection dialog
pub fn show(cursive: &mut Cursive, group_full_path: String) {
    // Create select
    let result = builders::build_directory_select(&group_full_path);

    match result {
        Ok(mut select) => {
            // Configure the next window
            select.set_on_submit(move |cursive_inst, selected_template: &str| {
                let template_full_path = format!("{group_full_path}/{selected_template}");
                dialog_template_config::show(cursive_inst, template_full_path);
            });
            // Add a layer
            let select_wraped = select.scrollable().full_screen();
            let dialog = wrappers::into_fullscreen_dialog(strings::TEMPLATES_TITLE, select_wraped);
            cursive.add_layer(dialog);
        }
        Err(e) => {
            let view =
                wrappers::into_fullscreen_layout(TextView::new(e.to_string()).align(Align::center()).full_screen());
            let dialog = wrappers::into_fullscreen_dialog(strings::TEMPLATES_TITLE, view);
            cursive.add_layer(dialog);
        }
    };
}
