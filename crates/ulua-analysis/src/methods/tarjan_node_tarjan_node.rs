use crate::{
  records::tarjan_node::TarjanNode,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TarjanNode {
  pub fn tarjan_node(
    &mut self,
    ty: TypeId,
    tp: TypePackId,
    on_stack: bool,
    dirty: bool,
    lowlink: i32,
  ) {
    self.ty = ty;
    self.tp = tp;
    self.on_stack = on_stack;
    self.dirty = dirty;
    self.lowlink = lowlink;
  }
}
