use alloc::format;
use core::ffi::c_char;

use ulua_common::{
  functions::is_analysis_flag_experimental::isAnalysisFlagExperimental, records::f_value::FValue,
};

pub fn set_luau_flags_default() {
  FValue::<bool>::set_all_unless(true, |name| {
    if !name.starts_with("Luau") {
      return true;
    }
    let name_c = format!("{name}\0");
    unsafe { isAnalysisFlagExperimental(name_c.as_ptr() as *const c_char) }
  });
}
