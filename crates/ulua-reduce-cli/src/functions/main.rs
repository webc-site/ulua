use std::process::exit;

use ulua_cli_lib::functions::read_file::read_file;

use crate::{functions::help::help, records::reducer::Reducer};

/// cpp `int main(int argc, char** argv)` (CLI/src/Reduce.cpp:483-513)
pub fn run(args: &[String]) {
  let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();

  if args.len() != 4 {
    help(&arg_refs);
  }

  // for (size_t i = 1; i < args.size(); ++i) if (args[i] == "--help") help(args);
  if args.iter().skip(1).any(|arg| arg == "--help") {
    help(&arg_refs);
  }

  let mut arg_iter = args.iter();
  let _program = arg_iter.next();
  let script_name = arg_iter.next().unwrap_or(&String::new()).clone();
  let app_name = arg_iter.next().unwrap_or(&String::new()).clone();
  let search_text = arg_iter.next().unwrap_or(&String::new()).clone();

  // std::optional<std::string> source = readFile(scriptName);
  let Some(source) = read_file(&script_name) else {
    // cpp 用 printf 输出到 stdout
    println!("Could not read source {script_name}");
    exit(1);
  };

  // Reducer reducer;
  let mut reducer = Reducer::new();

  // reducer.run(std::move(scriptName), std::move(appName), *source, searchText);
  reducer.run_string_string_string_view_string_view(script_name, app_name, &source, &search_text);
}
