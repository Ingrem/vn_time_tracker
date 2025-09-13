use crate::ui::ui_patterns::action_button;
use crate::{core::json_storage::load_sessions, ui::app::TimeTrackerApp};
use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};

/// Drow one session data
fn draw_session_row(ui: &mut egui::Ui, date: &str, duration: &str) {
    let text_color =
        if ui.visuals().dark_mode { Color32::from_rgb(200, 200, 200) } else { Color32::from_rgb(10, 10, 10) };

    ui.horizontal(|ui| {
        ui.label(RichText::new(date).size(16.0));
        ui.add_space(8.0);
        ui.label(RichText::new(duration).size(16.0).color(text_color));
    });
}

/// Drow sessions list for selected game
pub fn draw_sessions_window(app: &mut TimeTrackerApp, ctx: &egui::Context) {
    let mut window_open = app.state.show_sessions_window.is_some();

    if let Some(game_id) = app.state.show_sessions_window {
        if let Some(game) = app.state.games.iter().find(|g| g.id == game_id) {
            egui::Window::new(format!("Sessions: {}", game.name))
                .resizable(true)
                .default_size(Vec2::new(300.0, 200.0))
                .open(&mut window_open)
                .show(ctx, |ui| {
                    // --- Sessions list ---
                    let sessions = load_sessions(game.id);
                    let footer_height = 44.0; // for button
                    let scroll_height = (ui.available_height() - footer_height).max(0.0);

                    ScrollArea::vertical().max_height(scroll_height).show(ui, |ui| {
                        ui.set_min_width(ui.available_width());

                        if sessions.is_empty() {
                            ui.label(RichText::new("Sessions list is empty").size(18.0));
                        } else {
                            for s in &sessions {
                                draw_session_row(ui, &s.date, &s.duration);
                            }
                        }
                    });

                    // --- Footer with button ---
                    ui.separator();

                    if action_button(ui, " Close", Vec2::new(55.0, 30.0), None).clicked() {
                        app.state.show_sessions_window = None;
                    }
                });
        }
    }

    if !window_open {
        app.state.show_sessions_window = None;
    }
}
