use core::ffi::{c_char, c_int};

use ulua_code_gen::records::block_linearization_stats::BlockLinearizationStats;

pub use crate::functions::serialize_compile_stats::CFile;
use crate::macros::write_pair::WRITE_PAIR;

unsafe extern "C" {
  pub fn fprintf(fp: *mut CFile, format: *const c_char, ...) -> c_int;
}

pub mod libc {
  pub use super::fprintf;
}

#[repr(C)]
struct BlockLinearizationStatsRepr {
  const_prop_instruction_count: u32,
  time_seconds: f64,
}

pub(crate) unsafe fn serialize_block_linearization_stats(
  fp: *mut CFile,
  stats: &BlockLinearizationStats,
) {
  unsafe {
    let stats_repr =
      &*(stats as *const BlockLinearizationStats as *const BlockLinearizationStatsRepr);

    fprintf(fp, c"{\n".as_ptr());

    WRITE_PAIR!(
      fp,
      stats_repr,
      "                ",
      const_prop_instruction_count,
      "%u,\n"
    );
    WRITE_PAIR!(fp, stats_repr, "                ", time_seconds, "%f\n");

    fprintf(fp, c"            }".as_ptr());
  }
}
