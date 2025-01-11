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

use cursive::style::{BaseColor, Color, PaletteColor, Style};
use cursive::theme::{BorderStyle, Effect, Palette, PaletteStyle, Theme};
use once_cell::sync::Lazy;

pub const COLOR_APP_NAME: Color = BaseColor::Black.light();

pub static MAIN_THEME: Lazy<Theme> = Lazy::new(|| {
    let mut palette = Palette::retro();

    // Override some colors.
    palette[PaletteColor::Background] = Color::TerminalDefault;
    palette[PaletteColor::View] = Color::TerminalDefault;
    palette[PaletteColor::Primary] = BaseColor::White.light();
    palette[PaletteColor::TitlePrimary] = BaseColor::Cyan.dark();
    palette[PaletteColor::Secondary] = BaseColor::Cyan.light();
    palette[PaletteColor::Highlight] = BaseColor::Cyan.dark();

    // Override styles.
    palette[PaletteStyle::HighlightInactive] = Style::from(BaseColor::White.dark()).combine(Effect::Bold);
    palette[PaletteStyle::EditableTextCursor] = Style::from(BaseColor::Black.dark());

    Theme {
        shadow: false,
        borders: BorderStyle::None,
        palette,
    }
});
