use alloc::string::String;

/// 两段路径拼接（cpp `FileUtils.cpp:227` 两个 `joinPaths` 重载的合一）：
/// `lhs` 非空且不以分隔符结尾时补一个 `/`；`rhs_sep_matters` 表达两重载的
/// 唯一语义差——`const Ch*` 模板版还要求 `rhs` 不以分隔符开头（空 `rhs` 的
/// `'\0'` 通过该判定，照样补 `/`，对齐 cpp），`string_view` 版只看 `lhs` 尾部。
///
/// cpp 把首参当输出参数用（`joinPaths(out, lhs, rhs)`），Rust 版直接返回结果。
pub fn join_paths(lhs: &str, rhs: &str, rhs_sep_matters: bool) -> String {
  let separator = !lhs.is_empty()
    && !lhs.ends_with(['/', '\\'])
    && (!rhs_sep_matters || !rhs.starts_with(['/', '\\']));

  let mut result = String::with_capacity(lhs.len() + rhs.len() + usize::from(separator));
  result.push_str(lhs);
  if separator {
    result.push('/');
  }
  result.push_str(rhs);
  result
}
