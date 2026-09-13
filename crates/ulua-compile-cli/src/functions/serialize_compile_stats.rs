use core::ffi::{c_char, c_int, c_void};

use crate::{
  functions::serialize_lowering_stats::serialize_lowering_stats,
  macros::{write_name::WRITE_NAME, write_pair::WRITE_PAIR},
  records::compile_stats::CompileStats,
};

pub type CFile = c_void;

unsafe extern "C" {
  pub fn fprintf(fp: *mut CFile, format: *const c_char, ...) -> c_int;
}

pub mod libc {
  pub use super::fprintf;
}

pub(crate) unsafe fn serialize_compile_stats(fp: *mut CFile, stats: &CompileStats) {
  unsafe {
    fprintf(fp, c"{\n".as_ptr());

    WRITE_PAIR!(fp, stats, "        ", lines, "%zu,\n");
    WRITE_PAIR!(fp, stats, "        ", bytecode, "%zu,\n");
    WRITE_PAIR!(fp, stats, "        ", bytecode_instruction_count, "%zu,\n");
    WRITE_PAIR!(fp, stats, "        ", codegen, "%zu,\n");
    WRITE_PAIR!(fp, stats, "        ", read_time, "%f,\n");
    WRITE_PAIR!(fp, stats, "        ", misc_time, "%f,\n");
    WRITE_PAIR!(fp, stats, "        ", parse_time, "%f,\n");
    WRITE_PAIR!(fp, stats, "        ", compile_time, "%f,\n");
    WRITE_PAIR!(fp, stats, "        ", codegen_time, "%f,\n");

    WRITE_NAME!(fp, "        ", lower_stats);
    serialize_lowering_stats(fp, &stats.lower_stats);

    fprintf(fp, c"\n    }".as_ptr());
  }
}
