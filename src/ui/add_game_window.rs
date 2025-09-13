use crate::{core::game_crud::add_new_game, ui::app::TimeTrackerApp, ui::ui_patterns};
use eframe::egui::{self, Context, Vec2};
use rfd::FileDialog;

/// Renders the "Add game" window.
pub fn draw_add_game_window(app: &mut TimeTrackerApp, ctx: &Context) {
    if !app.state.show_add_game_window {
        return;
    }

    egui::Window::new("Add game").collapsible(false).resizable(false).show(ctx, |ui| {
        // --- Game name input field ---
        ui.horizontal(|ui| {
            ui_patterns::labeled_text_edit(
                ui,
                "Game name:",
                &mut app.state.new_game_name,
                20.0,
                Vec2::new(200.0, 30.0),
                20.0,
            );
        });

        ui.separator();

        // --- Buttons ---
        ui.horizontal(|ui| {
            let spacing = 20.0;
            let button_size = Vec2::new(90.0, 25.0);
            let offset = 5.0;

            let (select_clicked, cancel_clicked) = ui_patterns::centered_two_buttons(
                ui,
                ("Select file", "Cancel"),
                button_size,
                spacing,
                offset,
                (None, None),
            );

            if select_clicked {
                if let Some(path) = FileDialog::new().pick_file() {
                    add_new_game(app, path.to_string_lossy().to_string());
                }
            }

            if cancel_clicked {
                app.state.show_add_game_window = false;
            }
        });
    });
}
