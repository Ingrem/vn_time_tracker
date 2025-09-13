use crate::core::game_crud::rename_game;
use crate::ui::app::GameUpdate;
use crate::{core::game_launch::start_game, ui::app::TimeTrackerApp, ui::ui_patterns::*};
use eframe::egui::{Align, FontId, Key, Label, Layout, RichText, ScrollArea, Sense, TextEdit, Ui, Vec2};
use egui_extras::{Column, TableBuilder, TableRow};
use std::sync::mpsc::Sender;

/// Draw single row for one game
fn draw_game_row(
    row: &mut TableRow,
    game: &mut crate::core::json_storage::Game,
    editing_name: &mut Option<u32>,
    updates: &Sender<GameUpdate>,
    on_sessions: impl FnOnce(u32),
    on_delete: impl FnOnce(u32),
) {
    // Start button
    row.col(|ui| {
        if centered_button(ui, "▶", Vec2::new(80.0, 25.0), Some(ButtonStyle::Success)) {
            start_game(game, updates.clone());
        }
    });

    // Game name
    row.col(|ui| {
        if editing_name.as_ref() == Some(&game.id) {
            let response = ui.add(
                TextEdit::singleline(&mut game.name).font(FontId::proportional(20.0)).desired_width(f32::INFINITY),
            );

            let commit = response.lost_focus()
                || ui.input(|i| i.key_pressed(Key::Enter))
                || (ui.input(|i| i.pointer.any_released()) && !response.has_focus());

            if commit {
                *editing_name = None;
                rename_game(game.id, game.name.clone());
            }
        } else {
            let label = Label::new(RichText::new(&game.name).size(20.0)).sense(Sense::click());
            if ui.add(label).double_clicked() {
                *editing_name = Some(game.id);
            }
        }
    });

    // Played hours
    row.col(|ui| {
        ui.label(RichText::new(&game.hours).size(20.0));
    });

    // Sessions button
    row.col(|ui| {
        if centered_button(ui, "Check", Vec2::new(70.0, 25.0), Some(ButtonStyle::Neutral)) {
            on_sessions(game.id);
        }
    });

    // Delete button
    row.col(|ui| {
        if centered_button(ui, "Delete", Vec2::new(70.0, 25.0), Some(ButtonStyle::Danger)) {
            on_delete(game.id);
        }
    });
}

/// Draws the main games table with actions: Start, Sessions, Delete.
pub fn draw_games_table(app: &mut TimeTrackerApp, ui: &mut Ui) {
    // Get updates about running games
    while let Ok(update) = app.updates_rx.try_recv() {
        if let Some(game) = app.state.games.iter_mut().find(|g| g.id == update.game_id) {
            game.hours = update.hours;
        }
    }
    // "Add game" button
    if action_button(ui, "➕ Add game", Vec2::new(150.0, 30.0), Some(ButtonStyle::Success)).clicked() {
        app.state.show_add_game_window = true;
    }

    ui.separator();
    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        if app.state.games.is_empty() {
            ui.label(RichText::new("Empty games list").size(24.0));
            return;
        }

        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::exact(80.0)) // Start button
            .column(Column::remainder()) // Game name
            .column(Column::exact(120.0)) // Played hours
            .column(Column::exact(90.0)) // Sessions
            .column(Column::exact(80.0)) // Delete
            .column(Column::exact(10.0)) // Spacer
            .body(|mut body| {
                // Header row
                body.row(30.0, |mut row| {
                    header_cell(&mut row, "Start");
                    row.col(|ui| {
                        ui.label(RichText::new("Game name").size(20.0).strong());
                    });
                    header_cell(&mut row, "Played hours");
                    header_cell(&mut row, "Sessions");
                    header_cell(&mut row, "Delete");
                    row.col(|ui| {
                        ui.label(RichText::new("").size(10.0));
                    });
                });

                // Game rows
                for game in &mut app.state.games {
                    body.row(30.0, |mut row| {
                        draw_game_row(
                            &mut row,
                            game,
                            &mut app.state.editing_name,
                            &app.updates_tx,
                            |id| app.state.show_sessions_window = Some(id),
                            |id| app.state.show_confirm_delete_window = Some(id),
                        );
                    });
                }
            });
    });

    ui.separator();
    ui.separator();
}
