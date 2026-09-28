use std::process::exit;

use crate::functions::parse_args::DEFAULT_SUMMARY_FILE;

pub fn display_help(argv0: &str) {
  println!("Usage: {argv0} [options] [file list]");
  println!();
  println!("Available options:");
  println!("  -h, --help: Display this usage message.");
  println!("  -O<n>: compile with optimization level n (default 1, n should be between 0 and 2).");
  println!("  -g<n>: compile with debug level n (default 1, n should be between 0 and 2).");
  println!(
    "  --fflags=<flags>: comma-separated list of fast flags to enable/disable (--fflags=true,false,LuauFlag1=true,LuauFlag2=false)."
  );
  println!(
    "  --summary-file=<filename>: file in which bytecode analysis summary will be recorded (default '{DEFAULT_SUMMARY_FILE}')."
  );

  exit(0);
}
