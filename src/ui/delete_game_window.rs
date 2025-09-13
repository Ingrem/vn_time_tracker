use crate::{core::game_crud::delete_game_and_sessions, ui::app::TimeTrackerApp, ui::ui_patterns};
use eframe::egui::{Context, RichText, Vec2, Window};

/// Renders the confirmation dialog for deleting a game and its sessions.
pub fn draw_confirm_delete_window(app: &mut TimeTrackerApp, ctx: &Context) {
    if let Some(game_id) = app.state.show_confirm_delete_window {
        if let Some(game) = app.state.games.iter().find(|g| g.id == game_id).cloned() {
            Window::new(format!("Delete Game: {}", game.name))
                .resizable(false)
                .collapsible(false)
                .default_size([500.0, 100.0])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        // Confirmation message
                        ui.label(
                            RichText::new(format!(
                                "Are you sure you want to DELETE GAME and all sessions for\n'{}' ?",
                                game.name
                            ))
                            .size(18.0)
                            .strong(),
                        );

                        ui.separator();

                        // Buttons row
                        ui.horizontal(|ui| {
                            let (yes, no) = ui_patterns::centered_two_buttons(
                                ui,
                                ("Yes", "No"),
                                Vec2::new(70.0, 30.0),
                                20.0,
                                5.0,
                                (None, None),
                            );

                            if yes {
                                delete_game_and_sessions(game_id, app);
                            }
                            if no {
                                app.state.show_confirm_delete_window = None;
                            }
                        });
                    });
                });
        }
    }
}
