pub fn display_help(argv0: &str) {
  println!("Usage: {} [--mode] [options] [file list]", argv0);
  println!();
  println!("Available modes:");
  println!("  omitted: typecheck and lint input files");
  println!("  --annotate: typecheck input files and output source with type annotations");
  println!();
  println!("Available options:");
  println!("  --formatter=plain: report analysis errors in Luacheck-compatible format");
  println!("  --formatter=gnu: report analysis errors in GNU-compatible format");
  println!("  --mode=strict: default to strict mode when typechecking");
  println!("  --solver={{new|old}}: selects which typechecker to use (defaults to the new solver)");
  println!("  --timetrace: record compiler time tracing information into trace.json");
}
