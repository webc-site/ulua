use alloc::{format, string::String};

use crate::records::{function_counters::FunctionCounters, module_counters::ModuleCounters};

// Faithful port of Counters.cpp's `countersFunctionCallback`.
/// VM 裸指针（context/function）由 `counters_dump.rs` 的 `function_callback_cb`
/// C-ABI 外壳单点转成本地借用后转调，本函数为纯安全逻辑。
///
/// `function` 以 `Option` 区分 cpp 的 null 哨兵：null 时按行号给出 `<main>` /
/// `<anonymous>` 标记；非 null 时为 VM 函数名（lossy 文本）。
pub(crate) fn counters_function_callback(
  counters: &mut ModuleCounters,
  function: Option<&str>,
  line_defined: i32,
) {
  let name: String = match function {
    None if line_defined == 1 => String::from("<main>"),
    None => format!("<anonymous>:{line_defined}"),
    Some(function) => format!("{function}:{line_defined}"),
  };

  counters.functions.push(FunctionCounters {
    name,
    ..Default::default()
  });
}
