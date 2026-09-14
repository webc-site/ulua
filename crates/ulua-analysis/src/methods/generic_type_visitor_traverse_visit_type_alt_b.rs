//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/VisitType.h:444:generic_type_visitor_traverse`
//! Source: `Analysis/include/Luau/VisitType.h:444-510` (hand-ported)
//!
//! C++ `void GenericTypeVisitor<S>::traverse(TypePackId tp)`. Match over
//! `TypePackVariant` replaces the C++ `get<T>` if-chain (exhaustive, same
//! dispatch). NOTE: the skeleton's `TypePackVariant::Error` carries `unifiable::Error<TypePackId>`
//! variant (it dropped `Unifiable::Error`'s fields), so the error visit is
//! handed a default-constructed `ErrorTypePack` — revisit if a visitor ever
//! reads `.index`/`.synthetic` off a pack error.

use core::ffi::c_void;

use crate::{
  records::generic_type_visitor::{GenericTypeVisitorTrait, VisitSeen},
  type_aliases::{
    bound_type_pack::BoundTypePack, error_type_pack::ErrorTypePack, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};

pub fn traverse_type_pack_id<V: GenericTypeVisitorTrait>(this: &mut V, tp: TypePackId) {
  // tp 是 arena 句柄，遍历期间类型图不被本访问者修改（C++ 同契约；
  // VisitType.h:444 的 traverse(TypePackId) 无 RecursionLimiter）。
  // 句柄解引用收口在下方私有 helper，公共方法保持 safe。
  traverse_type_pack_id_in_limiter(this, tp)
}

fn traverse_type_pack_id_in_limiter<V: GenericTypeVisitorTrait>(this: &mut V, tp: TypePackId) {
  unsafe {
    if this.visitor_base().seen.has_seen(tp as *const c_void) {
      this.cycle_type_pack_id(tp);
      return;
    }

    match &(*tp).ty {
      TypePackVariant::Bound(bound_to) => {
        let btv = BoundTypePack {
          bound_to: *bound_to,
        };
        if this.visit_type_pack_id_bound_type_pack(tp, &btv) {
          this.traverse_type_pack_id(*bound_to);
        }
      }
      TypePackVariant::Free(ftv) => {
        this.visit_type_pack_id_free_type_pack(tp, ftv);
      }
      TypePackVariant::Generic(gtv) => {
        this.visit_type_pack_id_generic_type_pack(tp, gtv);
      }
      TypePackVariant::Error(_) => {
        let etv = ErrorTypePack {
          index: 0,
          synthetic: None,
        };
        this.visit_type_pack_id_error_type_pack(tp, &etv);
      }
      TypePackVariant::TypePack(pack) => {
        if this.visit_type_pack_id_type_pack(tp, pack) {
          for &ty in &pack.head {
            this.traverse_type_id(ty);
          }

          if let Some(tail) = pack.tail {
            this.traverse_type_pack_id(tail);
          }
        }
      }
      TypePackVariant::Variadic(pack) => {
        if this.visit_type_pack_id_variadic_type_pack(tp, pack) {
          this.traverse_type_id(pack.ty);
        }
      }
      TypePackVariant::Blocked(btp) => {
        this.visit_type_pack_id_blocked_type_pack(tp, btp);
      }
      TypePackVariant::TypeFunctionInstance(tfitp) => {
        // TypeFunctionDepthCounter tfdc{&typeFunctionDepth};
        this.visitor_base().type_function_depth += 1;

        if this.visit_type_pack_id_type_function_instance_type_pack(tp, tfitp) {
          for &t in &tfitp.type_arguments {
            this.traverse_type_id(t);
          }

          for &t in &tfitp.pack_arguments {
            this.traverse_type_pack_id(t);
          }
        }

        this.visitor_base().type_function_depth -= 1;
      }
    }

    this.visitor_base().seen.unsee(tp as *const c_void);
  }
}
