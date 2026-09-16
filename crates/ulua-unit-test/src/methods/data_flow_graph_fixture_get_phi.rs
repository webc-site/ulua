use ulua_analysis::{
  functions::get_def::get_def_id, records::phi::Phi, type_aliases::def_id_def::DefId,
};

use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;

impl DataFlowGraphFixture {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn get_phi(&self, def: DefId) -> *const Phi {
    unsafe { get_def_id::<Phi>(def) }
  }
}
