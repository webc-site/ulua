use crate::{records::source_node::SourceNode, type_aliases::module_ptr_module::ModulePtr};

/// 限额缩放步长：超时折半 / 提速加倍共用同一因子 2。
const K_SCALE_FACTOR: f64 = 2.0;

pub fn apply_internal_limit_scaling(source_node: &mut SourceNode, module: ModulePtr, limit: f64) {
  if module.timeout {
    source_node.autocomplete_limits_mult /= K_SCALE_FACTOR;
  } else if module.check_duration_sec < limit / K_SCALE_FACTOR {
    source_node.autocomplete_limits_mult =
      (source_node.autocomplete_limits_mult * K_SCALE_FACTOR).min(1.0);
  }
}
