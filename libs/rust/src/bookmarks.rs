use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bookmark {
    pub id: String,
    pub title: String,
    pub url: String,
}

pub fn get_bookmarks(bookmark_type: &str) -> io::Result<Vec<Bookmark>> {
    let file_name = format!("bookmarks-{}.json", bookmark_type);
    let path = Path::new(&file_name);

    if path.exists() {
        let contents = fs::read_to_string(path)?;
        let bookmarks: Vec<Bookmark> = serde_json::from_str(&contents)?;
        Ok(bookmarks)
    } else {
        Ok(Vec::new())
    }
}

pub fn add_bookmark(bookmark_type: &str, new_bookmark: Bookmark) -> io::Result<()> {
    let mut bookmarks = get_bookmarks(bookmark_type)?;
    // Prevent adding duplicate bookmarks based on ID
    if !bookmarks.iter().any(|b| b.id == new_bookmark.id) {
        bookmarks.push(new_bookmark);
        save_bookmarks(bookmark_type, &bookmarks)?;
    }
    Ok(())
}

pub fn remove_bookmark(bookmark_type: &str, bookmark_id: &str) -> io::Result<()> {
    let mut bookmarks = get_bookmarks(bookmark_type)?;
    let initial_len = bookmarks.len();
    bookmarks.retain(|b| b.id != bookmark_id);
    if bookmarks.len() < initial_len {
        save_bookmarks(bookmark_type, &bookmarks)?;
    }
    Ok(())
}

fn save_bookmarks(bookmark_type: &str, bookmarks: &[Bookmark]) -> io::Result<()> {
    let file_name = format!("bookmarks-{}.json", bookmark_type);
    let path = Path::new(&file_name);
    let json_string = serde_json::to_string_pretty(bookmarks)?;
    let mut file = fs::File::create(path)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}
