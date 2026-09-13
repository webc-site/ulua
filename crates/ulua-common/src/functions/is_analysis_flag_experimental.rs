mod _inner {
  use core::ffi::{CStr, c_char};

  /// # Safety
  /// `flag` 必须是有效的以 nul 结尾的 C 字符串或 null。
  pub unsafe fn is_analysis_flag_experimental(flag: *const c_char) -> bool {
    if flag.is_null() {
      return false;
    }

    static K_LIST: &[&[u8]] = &[
      b"LuauInstantiateInSubtyping\0",
      b"LuauFixIndexerSubtypingOrdering\0",
      b"LuauSolverV2\0",
      b"UseNewLuauTypeSolverDefaultEnabled\0",
      b"LuauRefactorStringSemanticSubtyping\0",
    ];

    let flag_cstr = unsafe { CStr::from_ptr(flag) };
    let flag_bytes = flag_cstr.to_bytes_with_nul();

    K_LIST.contains(&flag_bytes)
  }
}

pub use _inner::{
  is_analysis_flag_experimental, is_analysis_flag_experimental as isAnalysisFlagExperimental,
};
