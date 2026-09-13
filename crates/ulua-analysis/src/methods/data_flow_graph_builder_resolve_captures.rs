use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{collect_operands::collect_operands, get_def::get_def_id},
  records::{data_flow_graph_builder::DataFlowGraphBuilder, phi::Phi},
  type_aliases::def_id_def::DefId,
};

impl DataFlowGraphBuilder {
  pub fn resolve_captures(&mut self) {
    for (_symbol, capture) in self.captures.iter() {
      let mut operands: Vec<DefId> = Vec::new();
      for &v in &capture.all_versions[capture.version_offset..] {
        collect_operands(v, &mut operands);
      }

      for capture_def in &capture.capture_defs {
        let phi_ptr = unsafe { get_def_id::<Phi>(*capture_def) } as *mut Phi;
        LUAU_ASSERT!(!phi_ptr.is_null());
        unsafe {
          LUAU_ASSERT!((*phi_ptr).operands.is_empty());
          (*phi_ptr).operands = operands.clone();
        }
      }
    }
  }
}
