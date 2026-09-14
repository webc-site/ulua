//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1193:type_pack_stringifier_stringify`
//! Source: `Analysis/src/ToString.cpp:1193-1218` (hand-ported)
//!
//! `tp->ty.valueless_by_exception()` has no Rust counterpart; that branch is
//! dropped.

use crate::{
  records::type_pack_stringifier::TypePackStringifier,
  type_aliases::{
    bound_type_pack::BoundTypePack, error_type_pack::ErrorTypePack, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};
impl TypePackStringifier {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  /// C++ `void stringify(TypePackId tp)` — the `Luau::visit` dispatch.
  pub(crate) fn stringify_type_pack_id(&mut self, tp: TypePackId) {
    unsafe {
      let state = &mut *self.state;
      let opts = &*state.opts;
      let result = &*state.result;
      if opts.max_type_length > 0 && result.name.len() > opts.max_type_length {
        return;
      }

      if let Some(p) = state.cycle_tp_names.find(&tp) {
        let name = p.clone();
        state.emit(name.as_str());
        return;
      }

      match &(*tp).ty {
        TypePackVariant::Bound(_) => {
          let btv = BoundTypePack {
            bound_to: match &(*tp).ty {
              TypePackVariant::Bound(b) => *b,
              _ => unreachable!(),
            },
          };
          self.operator_call_4(tp, &btv)
        }
        TypePackVariant::Error(_) => {
          let etv = ErrorTypePack {
            index: 0,
            synthetic: None,
          };
          self.operator_call_5(tp, &etv)
        }
        TypePackVariant::Free(ftv) => self.operator_call(tp, ftv),
        TypePackVariant::Generic(gtv) => self.operator_call_2(tp, gtv),
        TypePackVariant::TypePack(pack) => self.operator_call_7(tp, pack),
        TypePackVariant::Variadic(vtp) => self.operator_call_8(tp, vtp),
        TypePackVariant::Blocked(btp) => self.operator_call_3(tp, btp),
        TypePackVariant::TypeFunctionInstance(tfitp) => self.operator_call_6(tp, tfitp),
      }
    }
  }
}
