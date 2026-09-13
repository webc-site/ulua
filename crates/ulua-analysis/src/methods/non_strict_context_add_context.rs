use crate::{
  functions::collect_operands::collect_operands,
  records::non_strict_context::NonStrictContext,
  type_aliases::{def_id_def::DefId, type_id::TypeId},
};
impl NonStrictContext {
  pub fn add_context(&mut self, def: &DefId, ty: TypeId) {
    let mut defs: Vec<DefId> = Vec::new();
    collect_operands(*def, &mut defs);
    for def in defs {
      self.context.insert(def, ty);
    }
  }
}
