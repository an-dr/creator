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

use crate::environment;
use cursive::{
    align::Align,
    utils::markup::StyledString,
    view::{Resizable, Scrollable},
    views::{Dialog, TextView},
    Cursive, View,
};

use super::{builders, dialog_select_template, strings, theme, wrappers};

/// Shows Template type selection dialog
pub fn show(cursive: &mut Cursive) {
    fn build_app_name_string(text: &str) -> TextView {
        let mut sstr = StyledString::new();
        sstr.append_styled(text, theme::COLOR_APP_NAME);
        TextView::new("").align(Align::center()).content(sstr)
    }

    fn into_main_fullscreen_dialog(title: &str, view: impl View) -> Dialog {
        // Layout with app name and notes
        let mut layout = wrappers::into_fullscreen_layout(view);
        layout.add_child(build_app_name_string(strings::APP_NAME));
        layout.add_child(build_app_name_string(strings::HELP_HINT));
        Dialog::around(layout).title(title)
    }

    let template_storage_path = environment::get_storage_path();
    let result = builders::build_directory_select(&template_storage_path);

    match result {
        Ok(mut select) => {
            // Configure opening of the next dialog
            select.set_on_submit(move |cursive_inst: &mut Cursive, selected: &str| {
                let group_path = format!("{template_storage_path}/{selected}");
                dialog_select_template::show(cursive_inst, group_path);
            });
            let select_wrapped = select.align(Align::center()).scrollable().full_screen();
            let select_dialog = into_main_fullscreen_dialog(strings::GROUPS_TITLE, select_wrapped);
            cursive.add_layer(select_dialog);
        }
        Err(e) => {
            // Error message
            let text = TextView::new(e.to_string()).align(Align::center()).full_screen();
            let text_dialog = into_main_fullscreen_dialog(strings::GROUPS_TITLE, text);
            cursive.add_layer(text_dialog);
        }
    };
}
