use clap::{App, Arg, SubCommand};

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
        .short("t")
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
    .arg(Arg::with_name("exclude")
        .multiple(true)
        .long("exclude")
        .short("e")
        .help("exclude a file from one specific mod, or a mod and all of its files entirely, from being merged. To exclude a mod entirely use: `--exclude modname` and to exclude a specific file use `--exclude modname:path/to/file.ws` where the path is a relative path starting from the mod's /scripts/content/ folder.")
    )
    .subcommand(SubCommand::with_name("tree")
        .about("returns a tree of the conflicts in a json format. The tree lists of files that need merges and for each file, the mods that edit it.")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("PATH")
            .takes_value(true)
            .default_value("mods")
            .help("path to the mods folder that needs to be merged")
        )
        // .arg(Arg::with_name("output")
        //     .short("o")
        //     .long("output")
        //     .value_name("PATH")
        //     .takes_value(true)
        //     .default_value("mods/mod0000_MergedFiles")
        //     .help("path to the mergedfiles folder")
        // )
        .arg(Arg::with_name("source")
            .short("s")
            .long("source")
            .value_name("PATH")
            .takes_value(true)
            .default_value("content/content0/scripts")
            .help("path to the source scripts or the vanilla scripts")
        )
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
