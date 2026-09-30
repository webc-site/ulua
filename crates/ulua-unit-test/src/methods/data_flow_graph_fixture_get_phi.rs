use ulua_analysis::{
  functions::get_def::get_def_id, records::phi::Phi, type_aliases::def_id_def::DefId,
};

use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;

impl DataFlowGraphFixture {
  /// cpp `getDefId<Phi>(def)`：把 def 句柄按变体标签下转成 `Phi`。
  ///
  /// 返回 `Option<&Phi>`：值类型不是 phi（cpp `get_if<Phi>` 未命中）即 `None`，
  /// 不再用「空句柄」当哨兵；命中时引用指向 def 池里的 `Phi`——bump arena 分配、
  /// 块地址不移动，寿命覆盖整个 fixture，可跨断言持有，只读比对。
  pub fn get_phi(&self, def: DefId) -> Option<&'static Phi> {
    get_def_id::<Phi>(def)
  }
}
