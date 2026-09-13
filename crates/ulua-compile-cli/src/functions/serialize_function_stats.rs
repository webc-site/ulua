// EXTERNAL_CRATE_REQUIRED: libc - provides FILE and fprintf for native CLI output
use core::ffi::{c_char, c_int, c_void};

use ulua_code_gen::records::function_stats::FunctionStats;

pub fn serialize_function_stats(fp: *mut c_void, stats: &FunctionStats) {
  unsafe {
    unsafe extern "C" {
      fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    }

    fprintf(fp, c"                {\n".as_ptr());

    // The macros WRITE_PAIR_STRING, WRITE_PAIR, and WRITE_NAME are defined in this crate
    // and they internally call libc::fprintf. Since we cannot rely on the 'libc' crate
    // being linked, we must ensure the macros use the local extern "C" fprintf or
    // we must provide a wrapper. However, the macros are already translated and
    // fixed in the crate. Based on the example 'serializeBlockLinearizationStats',
    // we define fprintf locally.

    // We redefine the logic of the macros here to avoid the unresolved 'libc' dependency
    // in the pre-translated macros, or we assume the environment provides a way to call it.
    // Given the constraints and the failure, we will implement the serialization manually
    // using the local fprintf to ensure it compiles.

    fprintf(
      fp,
      c"                    \"name\": \"%s\",\n".as_ptr(),
      stats.name.as_ptr(),
    );
    fprintf(
      fp,
      c"                    \"line\": %d,\n".as_ptr(),
      stats.line,
    );
    fprintf(
      fp,
      c"                    \"bcode_count\": %u,\n".as_ptr(),
      stats.bcode_count,
    );
    fprintf(
      fp,
      c"                    \"ir_count\": %u,\n".as_ptr(),
      stats.ir_count,
    );
    fprintf(
      fp,
      c"                    \"asm_count\": %u,\n".as_ptr(),
      stats.asm_count,
    );
    fprintf(
      fp,
      c"                    \"asm_size\": %u,\n".as_ptr(),
      stats.asm_size,
    );

    fprintf(fp, c"                    \"bytecode_summary\": ".as_ptr());

    let nesting_limit = stats.bytecode_summary.len();

    if nesting_limit == 0 {
      fprintf(fp, c"[]".as_ptr());
    } else {
      fprintf(fp, c"[\n".as_ptr());
      for (i, counts) in stats.bytecode_summary.iter().enumerate() {
        fprintf(fp, c"                        [".as_ptr());
        for (j, count) in counts.iter().enumerate() {
          fprintf(fp, c"%u".as_ptr(), *count);
          if j < counts.len() - 1 {
            fprintf(fp, c", ".as_ptr());
          }
        }
        fprintf(fp, c"]".as_ptr());
        if i < nesting_limit - 1 {
          fprintf(fp, c",\n".as_ptr());
        }
      }
      fprintf(fp, c"\n                    ]".as_ptr());
    }

    fprintf(fp, c"\n                }".as_ptr());
  }
}
