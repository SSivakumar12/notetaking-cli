use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

const FILE_PATH: &str = "notes.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    id: u32,
    title: String,
    body: String,
    time: String,
}

pub fn read_notes() -> Vec<Note> {
    let file_content = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new())
}

pub fn write_notes(notes: &[Note]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(notes)?;
    let mut file = File::create(FILE_PATH)?;
    file.write_all(json.as_bytes())
}

pub fn add_note(title: String, body: String) -> io::Result<()> {
    // add a note to the JSON object with an incrementing id
    // if the most recent id is 4, the new note will have 
    let mut notes = read_notes();
    let new_id = notes.last().map_or(1, |note| note.id + 1);
    notes.push(Note {
        id: new_id,
        title,
        body,
        time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    });
    write_notes(&notes)?;
    println!("Note added successfully with ID: {}", new_id);
    Ok(())
}

pub fn remove_note(id: u32) -> io::Result<()> {
    // checks if an `id` that a user want to remove is in the JSON database
    // if it is then that particular JSON object and metadata is removed.
    let mut notes = read_notes();
    let original_len = notes.len();
    notes.retain(|note| note.id != id);
    if notes.len() == original_len {
        println!("No note found with ID: {}", id);
    } else {
        write_notes(&notes)?;
        println!("Note removed successfully!");
    }
    Ok(())
}

pub fn modify_note(id: u32, title: Option<String>, body: Option<String>) -> io::Result<()> {
    // identifies a matching id and updates the title and body of the note.
    let mut notes = read_notes();
    if let Some(note) = notes.iter_mut().find(|note| note.id == id) {
        if let Some(new_title) = title {
            note.title = new_title;
        }
        if let Some(new_body) = body {
            note.body = new_body;
        }

        write_notes(&notes)?;
        println!("Note updated successfully!");
    } else {
        println!("No note found with ID: {}", id);
    }
    Ok(())
}

pub fn list_notes() {
    let notes = read_notes();
    if notes.is_empty() {
        println!("No notes found!");
    } else {
        for note in notes {
            println!(
                "ID: {id}, Title: {title}, Body: {body}, Recent Update: {update}",
                id = note.id,
                title = note.title,
                body = note.body,
                update = note.time
            );
        }
    }
}
