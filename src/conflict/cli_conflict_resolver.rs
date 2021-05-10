use difference::{Changeset};

struct Conflict<'a> {
  original: &'a str,
  ours: &'a str,
  theirs: &'a str
}

pub fn prompt_conflicts_in_file(input: &str) -> String {
  let conflict_start = "<<<<<<< ours";
  let conflict_end = ">>>>>>> theirs";
  let original_start = "||||||| original";
  let original_end = "=======";
  let original_start_length = original_start.len();
  let original_end_length = original_end.len();
  
  let mut output: Vec<String> = Vec::new();

  let chunks: Vec<&str> = input
    .split(conflict_start)
    .map(|chunk| chunk.split(conflict_end))
    .flatten()
    .collect();

  for chunk in chunks {
    let original_start_index = chunk.find(original_start).unwrap_or(0);
    let original_end_index = chunk.find(original_end).unwrap_or(chunk.len());

    // there is no conflict in the chunk
    if original_start_index == 0 && original_end_index == chunk.len() {
      output.push(String::from(chunk));
      continue;
    }

    let conflict = Conflict {
      ours: &chunk[0..original_start_index],
      original: &chunk[original_start_index+original_start_length..original_end_index],
      theirs: &chunk[original_end_index+original_end_length..]
    };

    let result = prompt_conflict(conflict);
    output.push(String::from(result));
    
  }
  
  output.join("")
}

fn prompt_conflict(conflict: Conflict) -> &str {
  println!("{}", conflict.original);
  println!("{}", Changeset::new(conflict.ours, conflict.theirs, "\n"));

  conflict.original
}