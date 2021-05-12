use std::{ffi::OsStr, fs, str::FromStr};
use std::path::{PathBuf};
use std::error::Error;

mod resolver;
mod conflict;
mod cli;
mod merge;

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

  let mods = fs::read_dir(input_path)?;

  for mod_result in mods {
    if let Ok(mod_name) = mod_result {
      let mod_path = mod_name.path();

      if !mod_path.file_name().unwrap_or(OsStr::new("")).to_str().unwrap_or("").starts_with("mod") {
        continue;
      }

      merge::merge::merge_mod(&origin_path, &output_path, &mod_path)?;
    }
  }

  Ok(())
}

