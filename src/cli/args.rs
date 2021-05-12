use clap::{App, Arg};

pub struct Args {
  pub input: String,
  pub source: String,
  pub output: String,

  pub text_editor: Option<String>,
  
  pub clean: bool,
  pub json: bool
}

pub fn get_args_match() -> Args {
  let matches = App::new("tw3-script-merger")
    .version("1.0")
    .author("Aelto")
    .about("The Witcher 3 - script merger")
    .arg(Arg::with_name("input")
        .short("i")
        .long("input")
        .value_name("PATH")
        .takes_value(true)
        .default_value("mods")
        .help("path to the mods folder that needs to be merged")
    )
    .arg(Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("PATH")
        .takes_value(true)
        .default_value("mods/mod0000_MergedFiles")
        .help("path to the mergedfiles folder")
    )
    .arg(Arg::with_name("source")
        .short("s")
        .long("source")
        .value_name("PATH")
        .takes_value(true)
        .default_value("content/content0/scripts")
        .help("path to the source scripts or the vanilla scripts")
    )
    .arg(Arg::with_name("texteditor")
        .short("te")
        .long("texteditor")
        .value_name("PATH")
        .takes_value(true)
        .help("path to the text editor that will be used to resolve the conflicts. Passing this parameter disable the JSON output")
    )
    .arg(Arg::with_name("json")
        .long("json")
        .help("tells to output the merge conflicts in stdout in the JSON format, then watches the conflicting file until the conflicts are resolved")
    )
    .arg(Arg::with_name("clean")
        .short("c")
        .long("clean")
        .help("tells if the output directory should be removed before merging")
    )
    .get_matches();

    Args {
      source: matches.value_of("source").expect("could not get the source path").to_string(),
      input: matches.value_of("input").expect("could not get the input path").to_string(),
      output: matches.value_of("output").expect("could not get output path").to_string(),

      text_editor: matches.value_of("texteditor").map(|s| s.to_string()),
      clean: matches.occurrences_of("clean") > 0,
      json: matches.occurrences_of("json") > 0
    }
}
