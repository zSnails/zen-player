extern crate sqlite;
#[doc(hidden)]

use serde::{Serialize, Deserialize};
use home::home_dir;
use std::marker::Sync;
use std::fs;
use tauri::Window;

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

#[derive(Serialize, Deserialize)]
struct Colors {
    background: String,
    foreground: String,
    secondary_foreground: String,
}

#[derive(Serialize, Deserialize)]
struct Theme {
    name: String,
    cover: String,
    colors: Colors,
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

#[tauri::command]
fn get_themes() -> Vec<Theme> { //-> Vec<Theme> {
  let mut dir_path = String::from("");
  match home_dir() {
    Some(home_path) => dir_path.push_str(home_path.to_str().unwrap()),
    None => println!("Error!"),
  }
  dir_path = format!("{}/.config/zen-player/themes/", &dir_path);
  let paths = fs::read_dir(dir_path).unwrap();
  let mut themes: Vec<Theme> = Vec::with_capacity(5);
  for path in paths {
    println!("{}", path.as_ref().unwrap().path().display());
    let text = std::fs::read_to_string(format!("{}", path.unwrap().path().display())).unwrap(); 
    let theme: Theme = serde_json::from_str(&text).unwrap();
    themes.push(theme);
  }
  
  return themes;
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
    .invoke_handler(tauri::generate_handler![get_playlists, create_playlist, get_themes])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
