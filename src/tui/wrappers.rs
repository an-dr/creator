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

use cursive::{
    align::Align,
    theme::{BaseColor, BorderStyle},
    utils::markup::StyledString,
    view::Resizable,
    views::{Dialog, DummyView, LinearLayout, TextView, ThemedView},
    Cursive, View,
};

// Wraps in a layout with the provided title and view
pub fn into_fullscreen_layout(view: impl View) -> LinearLayout {
    let mut layout = LinearLayout::vertical();
    layout.add_child(DummyView::new().max_height(1)); // spacer
    layout.add_child(view);
    layout
}

/// Wraps by a dialog with borders and a Title inside borders
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

pub fn into_fullscreen_dialog(title: &str, view: impl View) -> Dialog {
    let layout = into_fullscreen_layout(view);
    Dialog::around(layout).title(title)
}
