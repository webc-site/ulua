use core::ffi::{c_char, c_int, c_void};

use ulua_code_gen::records::lowering_stats::LoweringStats;

use crate::{
  functions::{
    serialize_block_linearization_stats::{CFile, serialize_block_linearization_stats},
    serialize_function_stats::serialize_function_stats,
  },
  macros::{write_name::WRITE_NAME, write_pair::WRITE_PAIR},
};

unsafe extern "C" {
  pub fn fprintf(fp: *mut CFile, format: *const c_char, ...) -> c_int;
}

pub mod libc {
  pub use super::fprintf;
}

pub fn serialize_lowering_stats(fp: *mut c_void, stats: &LoweringStats) {
  let fp_file = fp as *mut CFile;
  unsafe {
    fprintf(fp_file, c"{\n".as_ptr());

    WRITE_PAIR!(fp_file, stats, "            ", total_functions, "%u,\n");
    WRITE_PAIR!(fp_file, stats, "            ", skipped_functions, "%u,\n");
    WRITE_PAIR!(fp_file, stats, "            ", spills_to_slot, "%d,\n");
    WRITE_PAIR!(fp_file, stats, "            ", spills_to_restore, "%d,\n");
    WRITE_PAIR!(
      fp_file,
      stats,
      "            ",
      max_spill_slots_used,
      "%u,\n"
    );
    WRITE_PAIR!(fp_file, stats, "            ", blocks_pre_opt, "%u,\n");
    WRITE_PAIR!(fp_file, stats, "            ", blocks_post_opt, "%u,\n");
    WRITE_PAIR!(
      fp_file,
      stats,
      "            ",
      max_block_instructions,
      "%u,\n"
    );
    WRITE_PAIR!(fp_file, stats, "            ", reg_alloc_errors, "%d,\n");
    WRITE_PAIR!(fp_file, stats, "            ", lowering_errors, "%d,\n");

    WRITE_NAME!(fp_file, "            ", block_linearization_stats);
    serialize_block_linearization_stats(fp_file, &stats.block_linearization_stats);
    fprintf(fp_file, c",\n".as_ptr());

    WRITE_NAME!(fp_file, "            ", functions);
    let function_count = stats.functions.len();

    if function_count == 0 {
      fprintf(fp_file, c"[]".as_ptr());
    } else {
      fprintf(fp_file, c"[\n".as_ptr());
      for (i, function) in stats.functions.iter().enumerate() {
        serialize_function_stats(fp, function);
        if i < function_count - 1 {
          fprintf(fp_file, c",\n".as_ptr());
        }
      }
      fprintf(fp_file, c"\n            ]".as_ptr());
    }

    fprintf(fp_file, c"\n        }".as_ptr());
  }
}
