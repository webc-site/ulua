use ulua_common::{
  functions::is_analysis_flag_experimental::is_analysis_flag_experimental, records::f_value::FValue,
};

pub(crate) fn set_luau_flag(name: &str, state: bool) {
  // 刻意收缩（DELIBERATE DEVIATION，对 cpp Flags.cpp:25 的无条件警告）：
  // Rust 注册表 ⊂ cpp 注册表——「上游有、本端口未移植」的名字若也警告会误报，
  // 故仅对 Luau 前缀且非实验性的未知名出 Warning。
  if !FValue::<bool>::set_flag_by_name(name, state)
    && name.starts_with("Luau")
    && !is_analysis_flag_experimental(name)
  {
    eprintln!("Warning: unrecognized flag '{name}'.");
  }
}
