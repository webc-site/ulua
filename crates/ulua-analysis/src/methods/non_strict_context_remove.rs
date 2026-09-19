use crate::{
  functions::collect_operands::collect_operands, records::non_strict_context::NonStrictContext,
  type_aliases::def_id_def::DefId,
};
impl NonStrictContext {
  pub fn remove(&mut self, def: &DefId) -> bool {
    let mut defs: Vec<DefId> = Vec::new();
    collect_operands(*def, &mut defs);
    let mut result = true;
    for def in defs {
      let erased = self.context.remove(&def);
      result = result && erased.is_some();
    }
    result
  }
}
