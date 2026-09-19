use ulua_common::{
  functions::is_analysis_flag_experimental::is_analysis_flag_experimental, records::f_value::FValue,
};

pub fn set_luau_flag(name: &str, state: bool) {
  // SAFETY: CLI `--fflags` 解析发生在启动期，尚未创建工作线程
  if !unsafe { FValue::<bool>::set_flag_by_name(name, state) }
    && name.starts_with("Luau")
    && !is_analysis_flag_experimental(name)
  {
    eprintln!("Warning: unrecognized flag '{name}'.");
  }
}
