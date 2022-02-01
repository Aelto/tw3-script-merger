use std::fs;
use diffy::merge;
use encoding_rs_io::DecodeReaderBytes;
use std::io::Read;
use walkdir::WalkDir;
use std::process::Command;
use std::path::Path;
use std::error::Error;

use crate::conflict::{prompt_conflicts_in_file, print_json_conflict};
use crate::resolver;
use crate::cli;

/// merge the supplied mod where:
///  - A is vanilla
///  - B is the merge folder
///  - C is the mod
/// If the files from C don't exist in B then it will straight up copy the files
/// without merging them.
pub fn merge_mod(origin: &Path, output: &Path, modfolder: &Path) -> Result<(), Box<dyn Error>> {
  let content_folder = modfolder
    .join("content")
    .join("scripts");

  let modname = modfolder.file_name().unwrap().to_str().unwrap();

  if !content_folder.is_dir() {
    return Ok(());
  }

  for file in WalkDir::new(&content_folder)
    .follow_links(true)
    .into_iter()
    .filter_map(Result::ok)
    .filter(|e| !e.file_type().is_dir()) {

    let relative_path = file.path().strip_prefix(&content_folder)?;
    let origin_file_path = origin.join(relative_path);

    if !origin_file_path.is_file() {
      continue;
    }

    // the path to the file in the output folder
    let output_file_path = output.join(relative_path);

    if !output_file_path.is_file() {
      fs::create_dir_all(&output_file_path.parent().expect("msg"))?;
      fs::copy(file.path(), output_file_path)?;

      continue;
    }
    
    let origin_file_path = origin.join(relative_path);
    try_merge(&origin_file_path, &output_file_path, &file.path(), modname)?;
  }

  Ok(())
}

fn try_merge(origin_file_path: &Path, output_file_path: &Path, mod_path: &Path, modname: &str) -> Result<(),Box<dyn Error>> {
  let origin_content = get_string_from_file(&origin_file_path)?;
  let output_content = get_string_from_file(&output_file_path)?;
  let modded_content = get_string_from_file(&mod_path)?;


  let result = merge(&origin_content, &output_content, &modded_content);
  println!("merge {} and {}", &modname, &output_file_path.file_name().unwrap().to_str().unwrap());
  match result {
    Ok(v) => {
      fs::write(&output_file_path, v)?;
    },
    Err(v) => {
      println!("conflict between {} and {}", &modname, &output_file_path.file_name().unwrap().to_str().unwrap());

      match resolver::resolve_conflicts_in_file(&v) {

        // all conflicts were resolved automatically
        Ok(content) => {
          fs::write(&output_file_path, &content)?;
        },

        // not all conflicts were resolved
        Err(content) => show_conflicts_to_user(&output_file_path, &content, &modname)?
      }
    },
  };

  Ok(())
}

fn show_conflicts_to_user(output_file_path: &Path, content: &str, modname: &str) -> Result<(),Box<dyn Error>> {
  let args = cli::args::get_args_match();

  if let Some(text_editor_path) = args.text_editor {
    fs::write(&output_file_path, &content)?;

    open_conflict_with_text_editor(&output_file_path, &text_editor_path);
  }
  else if args.json {
    print_json_conflict(&content, &output_file_path, modname);
  }
  else {
    let content = prompt_conflicts_in_file(&content);

    fs::write(&output_file_path, &content)?;
  }

  Ok(())
}

fn open_conflict_with_text_editor(file_path: &Path, text_editor: &str) {
  if cfg!(target_os = "windows") {
    Command::new("cmd")
    .args(&["/c", text_editor, "--wait", &file_path.to_str().unwrap()])
    .output()
    .expect("error when opening the text editor");
  } else {
    Command::new("sh")
    .args(&["/c", text_editor, "--wait", &file_path.to_str().unwrap()])
    .output()
    .expect("error when opening the text editor");
  };
}

fn get_string_from_file(path: &Path) -> Result<String, Box<dyn Error>> {
  let source_data = fs::read(path)?;
  // N.B. `source_data` can be any arbitrary io::Read implementation.
  let mut decoder = DecodeReaderBytes::new(&source_data[..]);

  let mut dest = String::new();
  // decoder implements the io::Read trait, so it can easily be plugged
  // into any consumer expecting an arbitrary reader.
  decoder.read_to_string(&mut dest)?;

  Ok(dest)
}