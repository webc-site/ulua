use ulua_code_gen::enums::code_gen_counter::CodeGenCounter;

use crate::records::module_counters::ModuleCounters;

// Faithful port of Counters.cpp's `countersValueCallback`.
/// VM 裸 context 由 `counters_dump.rs` 的 `value_callback_cb` C-ABI 外壳转成
/// 本地独占借用后转调，本函数为纯安全逻辑。
///
/// 前置（外壳的 `# Safety` 契约承担）：`counters.functions` 已非空（须先由
/// `counters_function_callback` 压入当前函数，否则下方 `expect` panic）。
pub(crate) fn counters_value_callback(
  counters: &mut ModuleCounters,
  kind: i32,
  line: i32,
  hits: u64,
) {
  let function = counters
    .functions
    .last_mut()
    .expect("countersValueCallback called before countersFunctionCallback");

  let entry = function.counters.entry(line).or_default();

  // 计数种类是纯数据分派，全部在安全代码里完成
  match kind {
    k if k == CodeGenCounter::RegularBlockExecuted as i32 => entry.regular_executed += hits,
    k if k == CodeGenCounter::FallbackBlockExecuted as i32 => entry.fallback_executed += hits,
    k if k == CodeGenCounter::VmExitTaken as i32 => entry.vm_exit_taken += hits,
    _ => {}
  }
}
