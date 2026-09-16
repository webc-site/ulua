use crate::{records::source_node::SourceNode, type_aliases::module_ptr_module::ModulePtr};

pub fn apply_internal_limit_scaling(source_node: &mut SourceNode, module: ModulePtr, limit: f64) {
  if module.timeout {
    source_node.autocomplete_limits_mult /= 2.0;
  } else if module.check_duration_sec < limit / 2.0 {
    source_node.autocomplete_limits_mult = (source_node.autocomplete_limits_mult * 2.0).min(1.0);
  }
}
