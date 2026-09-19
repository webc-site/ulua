use ulua_common::{
  functions::is_default_enabled_flag::is_default_enabled_flag, records::f_value::FValue,
};

pub fn set_luau_flags_default() {
  // SAFETY: CLI 启动期参数处理，单线程
  unsafe { FValue::<bool>::set_all_unless(true, |name| !is_default_enabled_flag(name)) };
}
