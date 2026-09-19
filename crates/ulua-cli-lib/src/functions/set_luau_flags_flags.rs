use ulua_common::records::f_value::FValue;

pub fn set_luau_flags_bool(state: bool) {
  // SAFETY: CLI 启动期参数处理，单线程
  unsafe { FValue::<bool>::set_all_unless(state, |name| !name.starts_with("Luau")) };
}
