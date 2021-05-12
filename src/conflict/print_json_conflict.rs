use std::{cmp, path::PathBuf};
use serde::{Deserialize, Serialize};
use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;

#[derive(Serialize, Deserialize, Debug)]
struct Conflict {
  ours: String,
  original: String,
  theirs: String,

  // some of the code before and after the conflict
  context_before: String,
  context_after: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
  conflicts: Vec<Conflict>,
  file_name: String,
  file_path: String
}

pub fn print_json_conflict(input: &str, filepath: &PathBuf) {
  let conflict_start = "<<<<<<< ours";
  let conflict_end = ">>>>>>> theirs";
  let original_start = "||||||| original";
  let original_end = "=======";
  let original_start_length = original_start.len();
  let original_end_length = original_end.len();
  let context_size = 300;

  let filename = filepath.file_name()
      .map(|n| n.to_str().unwrap_or("unknown.ws"))
      .unwrap_or("unknown.ws");

  let mut message = Message {
    conflicts: Vec::new(),
    file_name: String::from(filename),
    file_path: String::from(filepath.to_str().unwrap_or(filename))
  };

  let mut slice = &input[..];

  loop {
    let start_index = slice.find(conflict_start);
    let length = slice.len();

    if start_index.is_none() {
      break;
    }

    let start_index = start_index.unwrap();
    let end_index = slice.find(conflict_end)
    .unwrap_or(length - 1);

    let original_start_index = slice.find(original_start)
      .unwrap_or(end_index);
    let original_end_index = slice.find(original_end)
      .unwrap_or(end_index);
    
    let left = if start_index < context_size {
      0
    } else {
      start_index - context_size
    };
    let right = cmp::min(end_index + conflict_end.len() + context_size, length - 1);
    
    message.conflicts.push(Conflict {
      ours: String::from(&slice[start_index + conflict_start.len() .. original_start_index]),
      original: String::from(&slice[original_start_index + original_start_length .. original_end_index]),
      theirs: String::from(&slice[original_end_index + original_end_length .. end_index]),

      context_before: String::from(&slice[left..start_index]),
      context_after: String::from(&slice[end_index + conflict_end.len() ..right])
    });

    slice = &slice[end_index + conflict_end.len()..];
  }

  println!("{}", serde_json::to_string(&message).unwrap());

  // now we watch for changes on the file
  let (tx, rx) = channel();

  // Create a watcher object, delivering raw events.
  // The notification back-end is selected based on the platform.
  let mut watcher = raw_watcher(tx).unwrap();

  // Add a path to be watched. All files and directories at that path and
  // below will be monitored for changes.
  watcher.watch(&filepath, RecursiveMode::NonRecursive).unwrap();

  match rx.recv() {
    // we don't care about what happens, we just wait until the file is
    // changed.
    _ => {}
  }
}
