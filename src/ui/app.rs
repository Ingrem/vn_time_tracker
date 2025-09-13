use crate::core::{icons, json_storage};
use crate::ui;
use eframe::{App, CreationContext, Frame, NativeOptions, Storage, egui};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};

/// Main app
#[derive(Serialize, Deserialize)]
pub struct PersistedState {
    pub games: Vec<json_storage::Game>,
    pub show_add_game_window: bool,
    pub new_game_name: String,
    pub editing_name: Option<u32>,
    pub show_sessions_window: Option<u32>,
    pub show_confirm_delete_window: Option<u32>,
}

pub struct GameUpdate {
    pub game_id: u32,
    pub hours: String,
}

pub struct TimeTrackerApp {
    pub state: PersistedState,
    pub updates_tx: Sender<GameUpdate>,
    pub updates_rx: Receiver<GameUpdate>,
}

impl Default for PersistedState {
    fn default() -> Self {
        Self {
            games: json_storage::load_games(),
            show_add_game_window: false,
            new_game_name: String::new(),
            editing_name: None,
            show_sessions_window: None,
            show_confirm_delete_window: None,
        }
    }
}

impl TimeTrackerApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let state = cc.storage.and_then(|storage| eframe::get_value(storage, eframe::APP_KEY)).unwrap_or_default();

        let (tx, rx) = channel::<GameUpdate>();

        Self { state, updates_tx: tx, updates_rx: rx }
    }
}

impl App for TimeTrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::main_window::draw_games_table(self, ui);
            ui::add_game_window::draw_add_game_window(self, ctx);
            ui::show_sessions_window::draw_sessions_window(self, ctx);
            ui::delete_game_window::draw_confirm_delete_window(self, ctx);
        });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }
}

/// Run app GUI
pub fn run_gui() -> eframe::Result<()> {
    let icon = Arc::new(icons::load_icon_from_bytes());

    let options = NativeOptions {
        viewport: egui::ViewportBuilder {
            inner_size: Some([700.0, 250.0].into()),
            min_inner_size: Some([700.0, 250.0].into()),
            icon: Some(icon),
            ..Default::default()
        },
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "VN Time Tracker",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(TimeTrackerApp::new(cc)))
        }),
    )
}
