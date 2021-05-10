
struct Conflict<'a> {
  original: &'a str,
  ours: &'a str,
  theirs: &'a str
}

/// this function takes the string content of a file with merge conflicts (git
/// style) and will try to resolve them.
/// It returns a Result<String, String> and is Ok(String) when all conflicts
/// were resolved and the String is the new content.
/// It return Err(String) when none or some of the conflicts were resolved but
/// there are still conflicts left in the file. The String is the semi-resolved
/// content.
pub fn resolve_conflicts_in_file(input: &str) -> Result<String, String> {
  let conflict_start = "<<<<<<< ours";
  let conflict_end = ">>>>>>> theirs";
  let original_start = "||||||| original";
  let original_end = "=======";
  let original_start_length = original_start.len();
  let original_end_length = original_end.len();
  
  let mut has_resolved_everything = true;
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
    match resolve_conflict(conflict) {
      Ok(resolved_chunk) => {
        output.push(String::from(resolved_chunk));
      }

      // it could not resolve the conflict
      Err(()) => {
        let mut c = conflict_start.to_owned();
        c.push_str(chunk);
        c.push_str(conflict_end);

        output.push(c);
        has_resolved_everything = false;
      }
    }
  }
  
  if has_resolved_everything {
    Ok(output.join(""))
  }
  else {
    Err(output.join(""))
  }
}

fn resolve_conflict(conflict: Conflict) -> Result<&str, ()> {

  // empty ours and original
  if conflict.ours.replace("\n", "").trim().len() == 0
  && conflict.original.replace("\n", "").trim().len() == 0 {
    return Ok(conflict.theirs);
  }

  // empty theirs and original
  if conflict.theirs.replace("\n", "").trim().len() == 0
  && conflict.original.replace("\n", "").trim().len() == 0 {
    return Ok(conflict.ours);
  }

  Err(())
}