use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub hours: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub game_id: u32,
    pub date: String,
    pub duration: String,
}

const GAMES_FILE: &str = "games.json";
const SESSIONS_FILE: &str = "sessions.json";

/// Generic helper to load a JSON file into a vector of T.
/// Returns an empty Vec if the file does not exist or parsing fails.
fn load_from_file<T: DeserializeOwned>(path: &str) -> Vec<T> {
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_default()
    } else {
        Vec::new()
    }
}

/// Generic helper to save a slice as pretty JSON.
fn save_to_file<T: Serialize>(path: &str, data: &[T]) {
    if let Ok(file) = OpenOptions::new().create(true).write(true).truncate(true).open(path) {
        let writer = BufWriter::new(file);
        let _ = serde_json::to_writer_pretty(writer, data);
    }
}

/// Loads all games from disk.
pub fn load_games() -> Vec<Game> {
    load_from_file(GAMES_FILE)
}

/// Saves all games to disk.
pub fn save_games(games: &[Game]) {
    save_to_file(GAMES_FILE, games);
}

/// Loads all sessions for a specific game.
pub fn load_sessions(game_id: u32) -> Vec<Session> {
    load_from_file::<Session>(SESSIONS_FILE).into_iter().filter(|s| s.game_id == game_id).collect()
}

/// Appends a new session to the session file.
pub fn save_session(session: &Session) {
    let mut sessions = load_from_file::<Session>(SESSIONS_FILE);
    sessions.push(session.clone());
    save_to_file(SESSIONS_FILE, &sessions);
}

/// Deletes all sessions belonging to a specific game.
/// Returns `true` if something was deleted.
pub fn delete_sessions_for_game(game_id: u32) -> bool {
    let mut sessions = load_from_file::<Session>(SESSIONS_FILE);
    let initial_len = sessions.len();
    sessions.retain(|s| s.game_id != game_id);

    if sessions.len() != initial_len {
        save_to_file(SESSIONS_FILE, &sessions);
        true
    } else {
        false
    }
}
