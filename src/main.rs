#![windows_subsystem = "windows"]

//! Entry point for the VN Time Tracker application.
//!
//! This crate uses [`eframe`] as the GUI framework and is split into three main modules:
//! - [`game_runner`]: manages the game loop or execution logic.
//! - [`storage`]: handles persistence (saving and loading data).
//! - [`ui`]: defines and runs the graphical user interface.

mod core;
mod ui;

/// Starts the application initializes and runs the `eframe` application loop.
fn main() -> eframe::Result<()> {
    ui::app::run_gui()
}
