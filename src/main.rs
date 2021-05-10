use std::{ffi::OsStr, fs, str::FromStr};
use conflict::prompt_conflicts_in_file;
use diffy::merge;
use std::error::Error;
use encoding_rs_io::DecodeReaderBytes;
use std::io::Read;
use std::env;
use std::path::{PathBuf};
use walkdir::WalkDir;
use std::process::Command;

mod resolver;
mod conflict;
mod cli;

fn main() -> Result<(), Box<dyn Error>> {
  let args = cli::args::get_args_match();

  let origin_path = PathBuf::from_str(&args.source).expect("could not parse the source path");
  let input_path = PathBuf::from_str(&args.input).expect("could not parse the input path");
  let output_path = PathBuf::from_str(&args.output).expect("could not parse the output path");

  if args.clean {
    fs::remove_dir_all(&output_path)?;
  }

  let mods = fs::read_dir(input_path)?;

  for mod_result in mods {
    if let Ok(mod_name) = mod_result {
      let mod_path = mod_name.path();

      // println!("merging mod {}", &mod_path.to_str().expect("msg"));

      if !mod_path.file_name().unwrap_or(OsStr::new("")).to_str().unwrap_or("").starts_with("mod") {
        continue;
      }

      merge_mod(&origin_path, &output_path, &mod_path)?;
    }
  }

  Ok(())
}

/// merge the supplied mod where:
///  - A is vanilla
///  - B is the merge folder
///  - C is the mod
/// If the files from C don't exist in B then it will straight up copy the files
/// without merging them.
fn merge_mod(origin: &PathBuf, output: &PathBuf, modfolder: &PathBuf) -> Result<(), Box<dyn Error>> {
  let content_folder = modfolder
    .join("content")
    .join("scripts");

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

    let origin_content = get_string_from_file(&origin_file_path)?;
    let output_content = get_string_from_file(&output_file_path)?;
    let modded_content = get_string_from_file(&PathBuf::from(file.path()))?;

    let result = merge(&origin_content, &modded_content, &output_content);
    match result {
      Ok(v) => {
        fs::write(&output_file_path, v)?;
      },
      Err(v) => {

        match resolver::resolve_conflicts_in_file(&v) {

          // all conflicts were resolved automatically
          Ok(content) => {
            fs::write(&output_file_path, &content)?;
          },

          // not all conflicts were resolved
          Err(content) => {
            let args = cli::args::get_args_match();

            dbg!(&args.text_editor);

            if let Some(text_editor_path) = args.text_editor {
              fs::write(&output_file_path, &content)?;

              if cfg!(target_os = "windows") {
                Command::new("cmd")
                .args(&["/c", &text_editor_path, "--wait", &output_file_path.to_str().unwrap()])
                .output()
                .expect("error when opening the text editor");
              } else {
                Command::new("sh")
                .args(&["/c", &text_editor_path, "--wait", &output_file_path.to_str().unwrap()])
                .output()
                .expect("error when opening the text editor");
              };
            }
            else {
              let content = prompt_conflicts_in_file(&v);

              fs::write(&output_file_path, &content)?;
            }
          }
        }
      },
    };
  }

  Ok(())
}

fn get_string_from_file(path: &PathBuf) -> Result<String, Box<dyn Error>> {
  let source_data = fs::read(path)?;
  // N.B. `source_data` can be any arbitrary io::Read implementation.
  let mut decoder = DecodeReaderBytes::new(&source_data[..]);

  let mut dest = String::new();
  // decoder implements the io::Read trait, so it can easily be plugged
  // into any consumer expecting an arbitrary reader.
  decoder.read_to_string(&mut dest)?;

  Ok(dest)
}