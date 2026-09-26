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
  ///
  /// # Safety
  /// `def` 须为 null 或本 fixture 的 `DataFlowGraph`（`graph` 字段）产出的存活 `DefId`：
  /// 即由 `get_def`/`get_local_def` 从当前 module 的 AST 查得、且构建期已写入 def 池。
  /// 传空句柄按 `get_def_id` 的前置条件即为非法（内部要读 `(*def).v` 取标签），
  /// 传非本图句柄则可能读到不相干的 arena 节点。
  pub unsafe fn get_phi(&self, def: DefId) -> Option<&'static Phi> {
    // Safety: 契约保证 def 为 def 池存活句柄；get_def_id 只按 Variant tag 只读下转，
    // 未命中返回 null，as_ref 折为 None，不新增借用。
    unsafe { get_def_id::<Phi>(def).as_ref() }
  }
}
