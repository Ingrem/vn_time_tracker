use crate::core::json_storage::{Game, Session, load_games, save_games, save_session};
use crate::ui::app::GameUpdate;
use chrono::Local;
use std::{process::Command, sync::mpsc::Sender, thread, time::Instant};

/// Launch a game process asynchronously, track its playtime, and persist the data.
///
/// This function spawns a new thread to run the game located at `current_game.path`.
/// Once the game exits, it calculates the session duration, updates the stored total
/// playtime in the JSON database, saves a session record, and sends a `GameUpdate`
/// through the provided `updates` channel to notify the UI.
///
/// # Arguments
///
/// * `current_game` - Mutable reference to the game being launched. Only `hours` is updated after the game finishes.
/// * `updates` - Channel sender to notify the UI of updated game hours.
pub fn start_game(current_game: &mut Game, updates: Sender<GameUpdate>) {
    let game_id = current_game.id;
    let path = current_game.path.clone();

    thread::spawn(move || {
        let start = Instant::now();

        match Command::new(&path).spawn().and_then(|mut child| child.wait()) {
            Ok(_) => {
                let elapsed = start.elapsed().as_secs();

                // Load games from JSON and update the played time
                let mut stored_games = load_games();
                if let Some(stored_game) = stored_games.iter_mut().find(|g| g.id == game_id) {
                    let total_secs = parse_duration(&stored_game.hours) + elapsed;
                    stored_game.hours = format_duration(total_secs);

                    let updated_hours = stored_game.hours.clone();

                    save_games(&stored_games);

                    // Save session
                    let session = Session {
                        game_id,
                        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        duration: format_duration(elapsed),
                    };
                    save_session(&session);

                    // Notify UI
                    let _ = updates.send(GameUpdate { game_id, hours: updated_hours.clone() });
                }
            }
            Err(err) => eprintln!("Failed to launch game {}: {:?}", path, err),
        }
    });
}

/// Parses a duration string of the form `"Xh Ym Zs"` into total seconds.
///
/// Returns `0` for invalid or missing parts.
fn parse_duration(s: &str) -> u64 {
    let mut h = 0;
    let mut m = 0;
    let mut sec = 0;

    for part in s.split_whitespace() {
        if let Some(val) = part.strip_suffix('h') {
            h = val.parse().unwrap_or(0);
        } else if let Some(val) = part.strip_suffix('m') {
            m = val.parse().unwrap_or(0);
        } else if let Some(val) = part.strip_suffix('s') {
            sec = val.parse().unwrap_or(0);
        }
    }

    h * 3600 + m * 60 + sec
}

/// Formats a duration in seconds into a human-readable string `"Xh Ym Zs"`.
fn format_duration(seconds: u64) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    format!("{}h {}m {}s", h, m, s)
}
