use std::process::exit;

pub fn display_help(argv0: &str) {
  println!("Usage: {} [options] [file list]", argv0);
  println!();
  println!("Available options:");
  println!("  -h, --help: Display this usage message.");
  println!("  -O<n>: compile with optimization level n (default 1, n should be between 0 and 2).");
  println!("  -g<n>: compile with debug level n (default 1, n should be between 0 and 2).");
  println!(
    "  --fflags=<flags>: comma-separated list of fast flags to enable/disable (--fflags=true,false,LuauFlag1=true,LuauFlag2=false)."
  );
  println!(
    "  --summary-file=<filename>: file in which bytecode analysis summary will be recorded (default 'bytecode-summary.json')."
  );

  exit(0);
}
