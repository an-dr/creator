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

mod app_config;
mod creator;
mod directory_analyzer;
mod environment;
mod tui;
use log::*;
use simplelog::*;
use std::fs::File;
use std::panic;

fn init_log() {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create("creator.log").unwrap(),
    )])
    .unwrap();

    debug!("Debug build: logging enabled");

    panic::set_hook(Box::new(|e| {
        error!("{e}");
    }));
}

fn run() {
    tui::run();
}

fn main() {
    if cfg!(debug_assertions) {
        init_log();
    }
    run();
}
