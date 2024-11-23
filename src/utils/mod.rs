use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

// deriving de/serialise to read and save as json objects
#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    id: u32,
    title: String,
    body: String,
    time: String,
}

pub fn read_notes(file_path: &'static str) -> Vec<Note> {
    let file_content = fs::read_to_string(file_path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new())
}

pub fn write_notes(notes: &[Note], file_path: &'static str) -> io::Result<()> {
    let json = serde_json::to_string_pretty(notes)?;
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())
}

pub fn add_note(title: String, body: String, file_path: &'static str) -> io::Result<()> {
    // add a note to the JSON object with an incrementing id
    // if the most recent id is 4, the new note will have an id of 5
    let mut notes = read_notes(file_path);
    let new_id = notes.last().map_or(1, |note| note.id + 1);
    notes.push(Note {
        id: new_id,
        title,
        body,
        time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    });
    write_notes(&notes, file_path)?;
    println!("Note added successfully with ID: {}", new_id);
    Ok(())
}

pub fn remove_note(id: u32, file_path: &'static str) -> io::Result<()> {
    // checks if an `id` that a user want to remove is in the JSON database
    // if it is then that particular JSON object and metadata is removed.
    let mut notes = read_notes(file_path);
    let original_len = notes.len();
    notes.retain(|note| note.id != id);
    if notes.len() == original_len {
        println!("No note found with ID: {}", id);
    } else {
        write_notes(&notes, file_path)?;
        println!("Note removed successfully!");
    }
    Ok(())
}

pub fn modify_note(
    id: u32,
    title: Option<String>,
    body: Option<String>,
    file_path: &'static str,
) -> io::Result<()> {
    // identifies a matching id and updates the title and body of the note.
    let mut notes = read_notes(file_path);
    if let Some(note) = notes.iter_mut().find(|note| note.id == id) {
        if let Some(new_title) = title {
            note.title = new_title;
        }
        if let Some(new_body) = body {
            note.body = new_body;
        }

        write_notes(&notes, file_path)?;
        println!("Note updated successfully!");
    } else {
        println!("No note found with ID: {}", id);
    }
    Ok(())
}

pub fn list_notes(file_path: &'static str) {
    let notes = read_notes(file_path);
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

// ###############################################################
// ################# PLACEHOLDER TO UNIT TEST FUNCTION ###########
// ###############################################################

#[cfg(test)]
mod tests {
    use super::*;
    // pointing to a dummy test file
    const FILE_PATH: &'static str = "test.json";
    #[test]
    fn test_read_notes() {
        assert_eq!(read_notes(FILE_PATH).len(), 2);
    }

    #[test]
    #[ignore] // ignoring  since this test clashes with test_add_delete_note otherwise this test passes.
    fn test_modify_notes() {
        // testing modification to title works successfully
        let first_note_title: String = read_notes(FILE_PATH).get(0).unwrap().title.clone();
        let first_note_body: String = read_notes(FILE_PATH).get(0).unwrap().body.clone();

        let updated_title: String = read_notes(FILE_PATH).get(1).unwrap().title.clone();
        let updated_body: String = read_notes(FILE_PATH).get(1).unwrap().body.clone();

        let _ = modify_note(1, Some(updated_title), Some(updated_body), FILE_PATH);
        assert_ne!(
            first_note_title,
            read_notes(FILE_PATH).get(0).unwrap().title
        );

        // clean-up to reinstate the file to what it was
        let _ = modify_note(1, Some(first_note_title), Some(first_note_body), FILE_PATH);
    }

    #[test]
    fn test_add_delete_notes() {
        let _ = add_note(
            "test note".to_string(),
            "this is a test note to be added".to_string(),
            FILE_PATH,
        );
        assert_eq!(read_notes(FILE_PATH).len(), 3);

        let _ = remove_note(3, FILE_PATH);
    }
}
