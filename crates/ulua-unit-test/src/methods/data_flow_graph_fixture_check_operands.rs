use ulua_analysis::{records::phi::Phi, type_aliases::def_id_def::DefId};

use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;

impl DataFlowGraphFixture {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_operands(&self, phi: *const Phi, operands: Vec<DefId>) {
    unsafe {
      let phi_ref = &*phi;
      let mut operand_set: Vec<DefId> = Vec::new();
      for o in operands {
        operand_set.push(o);
      }
      assert_eq!(phi_ref.operands.len(), operand_set.len());
      for o in &phi_ref.operands {
        assert!(operand_set.contains(o));
      }
    }
  }
}
