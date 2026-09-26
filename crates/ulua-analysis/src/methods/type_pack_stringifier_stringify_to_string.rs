use crate::{
  functions::to_string_detailed_to_string::visit_pack_arms,
  records::{type_pack_stringifier::TypePackStringifier, type_stringifier::TypeStringifier},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypePackStringifier {
  /// C++ `void stringify(TypeId tv)`. 转发给 `TypeStringifier::stringify_type_id`；
  /// `tvs.state` 与 `self.state` 指向同一 `StringifierState`，转发期间不再触碰
  /// `self.state`，借用串行无重叠。
  pub fn stringify_type_id(&mut self, tv: TypeId) {
    let mut tvs = TypeStringifier { state: self.state };
    tvs.stringify_type_id(tv);
  }

  /// C++ `void stringify(TypePackId tp)` — the `Luau::visit` dispatch.
  pub(crate) fn stringify_type_pack_id(&mut self, tp: TypePackId) {
    let state = self.st();
    let opts = state.opts_mut();
    let result = state.result_mut();
    if opts.max_type_length > 0 && result.name.len() > opts.max_type_length {
      return;
    }

    if let Some(p) = state.cycle_tp_names.find(&tp) {
      let name = p.clone();
      state.emit(name.as_str());
      return;
    }

    visit_pack_arms(self, tp);
  }
}
