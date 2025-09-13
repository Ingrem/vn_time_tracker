use crate::core::json_storage::{Game, delete_sessions_for_game, load_games, save_games};
use crate::ui::app::TimeTrackerApp;

/// Add new game
pub fn add_new_game(app: &mut TimeTrackerApp, file_path: String) {
    let mut games = load_games();

    // Determine game name
    let mut game_name = app.state.new_game_name.trim().to_string();
    if game_name.is_empty() {
        if let Some(stem) = std::path::Path::new(&file_path).file_stem() {
            let stem_str = stem.to_string_lossy().trim().to_string();
            if !stem_str.is_empty() {
                game_name = stem_str;
            }
        }
    }

    // Create new game object
    let new_id = games.iter().map(|g| g.id).max().unwrap_or(0) + 1;
    let new_game = Game { id: new_id, name: game_name, path: file_path, hours: "0h 0m 0s".to_string() };

    // Save
    games.push(new_game);
    save_games(&games);

    // Update app state
    app.state.games = load_games();
    app.state.show_add_game_window = false;
    app.state.new_game_name.clear();
}

/// Removes a game and its associated sessions
pub fn delete_game_and_sessions(game_id: u32, app: &mut TimeTrackerApp) {
    let mut games = load_games();
    games.retain(|g| g.id != game_id);
    save_games(&games);

    let _ = delete_sessions_for_game(game_id);

    app.state.show_confirm_delete_window = None;
    app.state.games = games;
}

/// Rename game
pub fn rename_game(game_id: u32, new_name: String) {
    let mut games = load_games();

    if let Some(game) = games.iter_mut().find(|g| g.id == game_id) {
        game.name = new_name;
    }

    save_games(&games);
}
