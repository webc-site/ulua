use std::process::exit;

use ulua_cli_lib::functions::read_file::read_file;

use crate::{functions::help::help, records::reducer::Reducer};

/// cpp `int main(int argc, char** argv)` (CLI/src/Reduce.cpp:483-513)
pub fn run(args: &[String]) {
  let argv0 = args.first().map(String::as_str).unwrap_or("ulua-reduce");

  // argc != 4 → Syntax + exit(1)；len 校验与参数解构合一（help 返回 `!`）
  let [_, script_name, app_name, search_text] = args else {
    help(argv0);
  };

  // for (size_t i = 1; i < args.size(); ++i) if (args[i] == "--help") help(args);
  if args[1..].iter().any(|arg| arg == "--help") {
    help(argv0);
  }

  // std::optional<std::string> source = readFile(scriptName);
  let Some(source) = read_file(script_name) else {
    // cpp 用 printf 输出到 stdout
    println!("Could not read source {script_name}");
    exit(1);
  };

  // Reducer reducer;
  let mut reducer = Reducer::new();

  // reducer.run(std::move(scriptName), std::move(appName), *source, searchText);
  reducer.run_string_string_string_view_string_view(
    script_name.clone(),
    app_name.clone(),
    &source,
    search_text,
  );
}
