use ulua_analysis::{records::phi::Phi, type_aliases::def_id_def::DefId};

use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;

impl DataFlowGraphFixture {
  /// cpp `checkOperands(Phi*, vector<DefId>)`：断言 phi 的操作数集合恰为期望集合
  /// （长度相等且互相包含，顺序无关）。
  ///
  /// 形参 `&Phi` 由 `get_phi` 命中的 arena 存活节点给出；`operands` 收切片，
  /// 断言只读比对，不再为此复制一个 Vec。
  pub fn check_operands(&self, phi: &Phi, operands: &[DefId]) {
    assert_eq!(phi.operands.len(), operands.len());
    for o in &phi.operands {
      assert!(operands.contains(o));
    }
  }
}
