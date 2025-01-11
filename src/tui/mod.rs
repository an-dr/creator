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

use cursive::event::Key;
mod builders;
mod dialog_main;
mod dialog_select_template;
mod dialog_template_config;
mod strings;
mod theme;
mod wrappers;

/// Run the tui application
pub fn run() {
    let mut tui = cursive::CursiveRunnable::default();
    tui.add_global_callback(Key::Esc, |cursive_inst| {
        if cursive_inst.screen_mut().len() > 1 {
            cursive_inst.pop_layer();
        } else {
            cursive_inst.quit()
        }
    });
    tui.set_theme(theme::MAIN_THEME.clone());
    dialog_main::show(&mut tui);
    tui.run();
}
