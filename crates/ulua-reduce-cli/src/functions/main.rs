use std::{env::args, process::exit};

use ulua_cli_lib::functions::read_file::read_file;

use crate::{functions::help::help, records::reducer::Reducer};

pub fn main() {
  let args = args().collect::<Vec<_>>();

  if args.len() != 4 {
    let arg_refs = args.iter().map(|s: &String| s.as_str()).collect::<Vec<_>>();
    help(&arg_refs);
  }

  for i in 1..args.len() {
    if args[i] == "--help" {
      let arg_refs = args.iter().map(|s: &String| s.as_str()).collect::<Vec<_>>();
      help(&arg_refs);
    }
  }

  let script_name = args[1].clone();
  let app_name = args[2].clone();
  let search_text = args[3].clone();

  let source = read_file(&script_name);
  if source.is_none() {
    eprintln!("Could not read source {}", script_name);
    exit(1);
  }
  let source = source.unwrap();

  // Reducer reducer;
  let mut reducer = Reducer::new();

  // reducer.run(std::move(scriptName), std::move(appName), *source, searchText);
  reducer.run_string_string_string_view_string_view(script_name, app_name, &source, &search_text);
}
