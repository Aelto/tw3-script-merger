use std::error::Error;
use std::path::PathBuf;
use std::{ffi::OsStr, fs, str::FromStr};

mod cli;
mod conflict;
mod merge;
mod resolver;

fn main() -> Result<(), Box<dyn Error>> {
  let args = cli::args::get_args_match();

  let origin_path = PathBuf::from_str(&args.source).expect("could not parse the source path");
  let input_path = PathBuf::from_str(&args.input).expect("could not parse the input path");
  let output_path = PathBuf::from_str(&args.output).expect("could not parse the output path");

  if args.clean {
    if let Err(error) = fs::remove_dir_all(&output_path) {
      if !args.json {
        println!("{}", error);
      }
    }
  }

  fs::create_dir_all(&output_path)?;

  let mods = fs::read_dir(input_path)?;

  for mod_result in mods.flatten() {
    // if let Ok(mod_name) = mod_result {
    let mod_path = mod_result.path();

    let mod_name = mod_path.file_name().unwrap().to_str().unwrap();
    let is_ignored = args.ignored.iter().any(|name| mod_name == name);
    if is_ignored {
      println!("{:?} ignored", mod_name);

      continue;
    }

    if !mod_path
      .file_name()
      .unwrap_or_else(|| OsStr::new(""))
      .to_str()
      .unwrap_or("")
      .starts_with("mod")
    {
      continue;
    }

    merge::merge_mod(&origin_path, &output_path, &mod_path)?;
    // }
  }

  // when the option is enabled and it send JSON message for each conflict
  // we also send an empty message at the end to notifiy the merge is done.
  if args.json {
    conflict::print_empty_json_conflict();
  }

  Ok(())
}
