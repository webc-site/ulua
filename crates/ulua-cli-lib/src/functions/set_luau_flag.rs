use alloc::format;
use core::ffi::c_char;

use ulua_common::{
  functions::is_analysis_flag_experimental::isAnalysisFlagExperimental, records::f_value::FValue,
};

pub fn set_luau_flag(name: &str, state: bool) {
  if !FValue::<bool>::set_flag_by_name(name, state) {
    // 原行为: 只对 Luau 前缀且非 experimental 假 flag 告警
    let name_c = format!("{name}\0");
    if name.as_bytes().starts_with(b"Luau")
      && unsafe { !isAnalysisFlagExperimental(name_c.as_ptr() as *const c_char) }
    {
      eprintln!("Warning: unrecognized flag '{name}'.");
    }
  }
}
