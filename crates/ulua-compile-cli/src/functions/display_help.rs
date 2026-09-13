use core::ffi::{CStr, c_char};

pub fn display_help(argv0: *const c_char) {
  unsafe {
    // Use std::print! to avoid dependency on libc crate, matching the behavior of the C++ printf call
    let argv0_str = CStr::from_ptr(argv0).to_string_lossy();
    println!("Usage: {} [--mode] [options] [file list]", argv0_str);
    println!();
    println!("Available modes:");
    println!(
      "   binary, text, remarks, codegen, codegenir, codegenasm, codegenverbose, codegennull, null"
    );
    println!();
    println!("Available options:");
    println!("  -h, --help: Display this usage message.");
    println!(
      "  -O<n>: compile with optimization level n (default 1, n should be between 0 and 2)."
    );
    println!("  -g<n>: compile with debug level n (default 1, n should be between 0 and 2).");
    println!(
      "  --target=<target>: compile code for specific architecture (a64, x64, a64_nf, x64_ms)."
    );
    println!("  --timetrace: record compiler time tracing information into trace.json");
    println!(
      "  --record-stats=<granularity>: granularity of compilation stats (total, file, function)."
    );
    println!("  --bytecode-summary: Compute bytecode operation distribution.");
    println!("  --dump-constants: Dump constant table for each function (text mode only).");
    println!(
      "  --stats-file=<filename>: file in which compilation stats will be recored (default 'stats.json')."
    );
    println!("  --vector-lib=<name>: name of the library providing vector type operations.");
    println!("  --vector-ctor=<name>: name of the function constructing a vector value.");
    println!("  --vector-type=<name>: name of the vector type.");
    println!("  --only-parse: Only parse the input.");
    println!("  --parse-cst: Whether parser should parse CST in addition to AST.");
    println!(
      "  --fflags=<flags>: comma-separated list of fast flags to enable/disable (--fflags=true,false,LuauFlag1=true,LuauFlag2=false)."
    );
  }
}
