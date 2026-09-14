pub fn display_help(argv0: &str) {
  println!("Usage: {} [options] [file list] [-a] [arg list]", argv0);
  println!();
  println!("When file list is omitted, an interactive REPL is started instead.");
  println!();
  println!("Available options:");
  println!(
    "  --coverage: collect code coverage while running the code and output results to coverage.out"
  );
  println!(
    "  --counters: collect native counters data while running the code and output results to callgrind.out"
  );
  println!("  -h, --help: Display this usage message.");
  println!(
    "  -i, --interactive: Run an interactive REPL after executing the last script specified."
  );
  println!("  -O<n>: compile with optimization level n (default 1, n should be between 0 and 2).");
  println!("  -g<n>: compile with debug level n (default 1, n should be between 0 and 2).");
  println!(
    "  --profile[=N]: profile the code using N Hz sampling (default 10000) and output results to profile.out"
  );
  println!("  --timetrace: record compiler time tracing information into trace.json");
  println!("  --codegen: execute code using native code generation");
  println!(
    "  --codegen-cold: execute code using native code generation, including any functions deemed not profitable to natively compile"
  );
  println!(
    "  --codegen-perf: execute code using native code generation and profile using perf (only on Linux)"
  );
  println!("  --program-args,-a: declare start of arguments to be passed to the Luau program");
  println!(
    "  --fflags=<flags>: comma-separated list of fast flags to enable/disable (--fflags=true,false,LuauFlag1=true,LuauFlag2=false)."
  );
}
