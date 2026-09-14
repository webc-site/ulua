//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:375:type_stringifier_stringify`
//! Source: `Analysis/src/ToString.cpp:375-400` (hand-ported)
//!
//! C++ `void stringify(TypeId tv)` — the `Luau::visit` dispatch over the
//! type variant, calling the pinned `operator_call_N` arm per member.
//! `tv->ty.valueless_by_exception()` has no Rust counterpart (enums cannot
//! be valueless); that branch is dropped.

use crate::{
  records::type_stringifier::TypeStringifier,
  type_aliases::{bound_type::BoundType, type_id::TypeId, type_variant::TypeVariant},
};
impl TypeStringifier {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn stringify_type_id(&mut self, tv: TypeId) {
    unsafe {
      let state = &mut *self.state;
      let opts = &*state.opts;
      let result = &*state.result;
      if opts.max_type_length > 0 && result.name.len() > opts.max_type_length {
        return;
      }

      if let Some(p) = state.cycle_names.find(&tv) {
        let name = p.clone();
        state.emit(name.as_str());
        return;
      }

      match &(*tv).ty {
        TypeVariant::Bound(_) => {
          let btv = BoundType {
            bound_to: match &(*tv).ty {
              TypeVariant::Bound(b) => *b,
              _ => unreachable!(),
            },
          };
          self.operator_call_10(tv, &btv)
        }
        TypeVariant::Error(etv) => self.operator_call_11(tv, etv),
        TypeVariant::Free(ftv) => self.operator_call_2(tv, ftv),
        TypeVariant::Generic(gtv) => self.operator_call_3(tv, gtv),
        TypeVariant::Primitive(ptv) => self.operator_call_17(tv, ptv),
        TypeVariant::Singleton(stv) => self.operator_call_18(tv, stv),
        TypeVariant::Blocked(btv) => self.operator_call_9(tv, btv),
        TypeVariant::PendingExpansion(petv) => self.operator_call_6(tv, petv),
        TypeVariant::Function(ftv) => self.operator_call_12(tv, ftv),
        TypeVariant::Table(ttv) => self.operator_call_7(tv, ttv),
        TypeVariant::Metatable(mtv) => self.operator_call_5(tv, mtv),
        TypeVariant::Extern(etv) => self.operator_call(tv, etv),
        TypeVariant::Any(atv) => self.operator_call_8(tv, atv),
        TypeVariant::Union(utv) => self.operator_call_20(tv, utv),
        TypeVariant::Intersection(itv) => self.operator_call_4(tv, itv),
        TypeVariant::Lazy(ltv) => self.operator_call_13(tv, ltv),
        TypeVariant::Unknown(utv) => self.operator_call_21(tv, utv),
        TypeVariant::Never(ntv) => self.operator_call_15(tv, ntv),
        TypeVariant::Negation(ntv) => self.operator_call_14(tv, ntv),
        TypeVariant::NoRefine(nrt) => self.operator_call_16(tv, nrt),
        TypeVariant::TypeFunctionInstance(tfitv) => self.operator_call_19(tv, tfitv),
      }
    }
  }
}
