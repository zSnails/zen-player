extern crate sqlite;
// #[macro_use] extern crate serde_derive;
#[doc(hidden)]

use serde::Serialize;
use home::home_dir;
use std::marker::Sync;
use tauri::{Window};
// #![cfg_attr(
// //   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]

struct Database(sqlite::Connection);
unsafe impl Sync for Database {}

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
struct Playlist {
    id: i32,
    name: String,
    description: String,
    cover: String,
}

impl Playlist {
    fn new(id: i32, name: String, description: String, cover: String) -> Playlist {
        Playlist { id, name, description, cover }
    }
}

#[tauri::command]
fn get_playlists(database: tauri::State<'_, Database>) -> Vec<Playlist> {
    let mut cursor = database.0.prepare("SELECT * FROM playlists").unwrap().into_cursor();
    // cursor.bind();
    let mut playlists = Vec::with_capacity(cursor.column_count());
    while let Some(row) = cursor.next().unwrap() {
        playlists.push(Playlist::new(row[0].as_integer().unwrap() as i32, row[1].as_string().unwrap().to_string(), row[2].as_string().unwrap().to_string(), row[3].as_string().unwrap().to_string()));
    }

    playlists
}

#[tauri::command]
fn create_playlist(name: String, description: String, cover: String, database: tauri::State<'_, Database>, window: Window) {
    database.0.execute(format!("INSERT INTO playlists (name, description, cover) values ('{}', '{}', '{}')", name, description, cover)).unwrap();

    let mut cursor = database.0.prepare("SELECT last_insert_rowid()").unwrap().into_cursor();
    while let Some(row) = cursor.next().unwrap() {
        window.emit("backend:playlist-create", Playlist::new(row[0].as_integer().unwrap() as i32, name.to_owned(), description.to_owned(), cover.to_owned())).unwrap();
    }
}

fn main() {
  let mut path = String::from("");
  match home_dir() {
    Some(home_path) => path.push_str(home_path.to_str().unwrap()),
    None => println!("Yesn't"),
  }
  let connection = sqlite::open(format!("{}/data.zen", path)).unwrap();
  match connection.execute("CREATE TABLE playlists (id INTEGER PRIMARY KEY, name TEXT, description TEXT, cover TEXT)") {
    Ok(_) => println!("Created playlists table"),
    Err(_) => eprintln!("Table already exists, ignoring")
  }

  match connection.execute("CREATE TABLE songs (id INTEGER PRIMARY KEY, playlist_id INTEGER, name TEXT, cover TEXT, FOREIGN KEY (playlist_id) REFERENCES playlists(id))") {
    Ok(_) => println!("Created songs table"),
    Err(_) => eprintln!("Songs table already exists, ignoring")
  }
  tauri::Builder::default()
    .manage(Database(connection))
    .invoke_handler(tauri::generate_handler![get_playlists, create_playlist])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
