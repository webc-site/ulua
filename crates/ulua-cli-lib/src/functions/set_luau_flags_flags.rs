use ulua_common::records::f_value::FValue;

pub fn set_luau_flags_bool(state: bool) {
  FValue::<bool>::set_all_unless(state, |name| !name.starts_with("Luau"));
}
