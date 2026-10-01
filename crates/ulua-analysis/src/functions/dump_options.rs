use crate::records::to_string_options::ToStringOptions;

/// cpp `dumpOptions()`（`Analysis/src/ToString.cpp:1789`）的函数级
/// `static ToStringOptions options`。cpp 直接返回该静态的可变引用；Rust 里两个
/// 并发 `dump()` 会同时持有 `&'static mut`（别名 UB），且 `ToStringOptions` 因
/// `name_map` 以 `*const Type` 为键而非 `Sync`，故每次构造一份：dump 是调试路径，
/// 单份构造成本可忽略。
pub fn dump_options() -> ToStringOptions {
  let mut opts = ToStringOptions::new(true);
  opts.exhaustive = true;
  opts.function_type_arguments = true;
  opts.max_table_length = 0;
  opts.max_type_length = 0;
  opts
}
