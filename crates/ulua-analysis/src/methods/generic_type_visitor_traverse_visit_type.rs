use ulua_common::{fint, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{
    follow_type::follow,
    get_type::{get, type_variant_of},
    get_type_pack::type_pack_variant_of,
  },
  records::{
    generic_type_visitor::{GenericTypeVisitorTrait, VisitSeen},
    intersection_type::IntersectionType,
    union_type::UnionType,
  },
  type_aliases::{
    bound_type::BoundType, bound_type_pack::BoundTypePack, type_id::TypeId,
    type_pack_id::TypePackId, type_pack_variant::TypePackVariant, type_variant::TypeVariant,
  },
};

pub fn traverse_type_id<V: GenericTypeVisitorTrait>(this: &mut V, mut ty: TypeId) {
  // if (is<BoundType>(ty) && skipBoundTypes) ty = follow(ty);
  // 注意：follow 须先于 RecursionLimiter（VisitType.h:229），保证环类型入口一致。
  if get::<BoundType>(ty).is_some() && this.visitor_base().skip_bound_types {
    ty = follow(ty);
  }

  // RecursionLimiter limiter{visitorName, &recursionCounter, FInt::LuauVisitRecursionLimit};
  {
    let base = this.visitor_base();
    base.recursion_counter += 1;
    let limit = fint::LuauVisitRecursionLimit.get();
    if limit > 0 && base.recursion_counter > limit {
      panic!(
        "Internal recursion counter limit exceeded: {}",
        base.visitor_name
      );
    }
  }

  // SAFETY: ty 是 arena 句柄，遍历期间类型图不被本访问者修改（C++ 同契约）。
  traverse_type_id_in_limiter(this, ty);

  // ~RecursionLimiter (the C++ dtor decrement).
  this.visitor_base().recursion_counter -= 1;
}

/// 变体读取收口在 `type_variant_of`：公共 `traverse_type_id` 保持 safe。
fn traverse_type_id_in_limiter<V: GenericTypeVisitorTrait>(this: &mut V, ty: TypeId) {
  // `ty` 是调用方 arena 持有的存活 `Type` 句柄（C++ `traverse(TypeId)` 同契约，
  // 收口于 `type_variant_of`）；本访问者遍历期间类型图不被其自身改写；
  // `seen.has_seen` 只做去重记账，与 `this` 的可变借用（访问者状态）互不别名。
  if this.visitor_base().seen.has_seen(ty as *const ()) {
    this.cycle_type_id(ty);
    return;
  }

  match type_variant_of(ty) {
    TypeVariant::Bound(_) => {
      // At this point, we know that `skipBoundTypes` is false, as
      // otherwise we would have hit the above branch.
      LUAU_ASSERT!(!this.visitor_base().skip_bound_types);
      // TypeVariant::Bound 已命中，get 必返回 Some（C++ 必真分支）。
      let Some(btv) = get::<BoundType>(ty) else {
        return;
      };
      if this.visit_type_id_bound_type(ty, btv) {
        traverse_type_id(this, btv.bound_to);
      }
    }
    TypeVariant::Free(ftv) => {
      if this.visit_type_id_free_type(ty, ftv) {
        // Regardless of the choice of solver, all free types are guaranteed to have
        // lower and upper bounds
        LUAU_ASSERT!(!ftv.lower_bound.is_null());
        LUAU_ASSERT!(!ftv.upper_bound.is_null());

        traverse_type_id(this, ftv.lower_bound);
        traverse_type_id(this, ftv.upper_bound);
      }
    }
    TypeVariant::Generic(gtv) => {
      this.visit_type_id_generic_type(ty, gtv);
    }
    TypeVariant::Error(etv) => {
      this.visit_type_id_error_type(ty, etv);
    }
    TypeVariant::Primitive(ptv) => {
      this.visit_type_id_primitive_type(ty, ptv);
    }
    TypeVariant::Function(ftv) => {
      if this.visit_type_id_function_type(ty, ftv) {
        this.traverse_type_pack_id(ftv.arg_types);
        this.traverse_type_pack_id(ftv.ret_types);
      }
    }
    TypeVariant::Table(ttv) => {
      // Some visitors want to see bound tables, that's why we traverse the original type
      if this.visitor_base().skip_bound_types
        && let Some(bound_to) = ttv.bound_to
      {
        traverse_type_id(this, bound_to);
      } else if this.visit_type_id_table_type(ty, ttv) {
        if let Some(bound_to) = ttv.bound_to {
          traverse_type_id(this, bound_to);
        } else {
          for prop in ttv.props.values() {
            if let Some(read_ty) = prop.read_ty {
              traverse_type_id(this, read_ty);
            }

            // In the case that the readType and the writeType
            // are the same pointer, just traverse once.
            // Traversing each property twice has pretty
            // significant performance consequences.
            if let Some(write_ty) = prop.write_ty
              && !prop.is_shared()
            {
              traverse_type_id(this, write_ty);
            }
          }

          if let Some(indexer) = &ttv.indexer {
            traverse_type_id(this, indexer.index_type);
            traverse_type_id(this, indexer.index_result_type);
          }
        }
      }
    }
    TypeVariant::Metatable(mtv) => {
      if this.visit_type_id_metatable_type(ty, mtv) {
        traverse_type_id(this, mtv.table);
        traverse_type_id(this, mtv.metatable);
      }
    }
    TypeVariant::Extern(etv) => {
      if this.visit_type_id_extern_type(ty, etv) {
        for prop in etv.props.values() {
          if let Some(read_ty) = prop.read_ty {
            traverse_type_id(this, read_ty);
          }

          // In the case that the readType and the writeType are
          // the same pointer, just traverse once. Traversing each
          // property twice would have pretty significant
          // performance consequences.
          if let Some(write_ty) = prop.write_ty
            && !prop.is_shared()
          {
            traverse_type_id(this, write_ty);
          }
        }

        if let Some(parent) = etv.parent {
          traverse_type_id(this, parent);
        }

        if let Some(metatable) = etv.metatable {
          traverse_type_id(this, metatable);
        }

        if let Some(indexer) = &etv.indexer {
          traverse_type_id(this, indexer.index_type);
          traverse_type_id(this, indexer.index_result_type);
        }
      }
    }
    TypeVariant::Any(atv) => {
      this.visit_type_id_any_type(ty, atv);
    }
    TypeVariant::NoRefine(nrt) => {
      this.visit_type_id_no_refine_type(ty, nrt);
    }
    TypeVariant::Union(utv) => {
      if this.visit_type_id_union_type(ty, utv) {
        let mut union_changed = false;
        for &opt_ty in utv.options.iter() {
          traverse_type_id(this, opt_ty);
          if get::<UnionType>(follow(ty)).is_none() {
            union_changed = true;
            break;
          }
        }

        if union_changed {
          traverse_type_id(this, ty);
        }
      }
    }
    TypeVariant::Intersection(itv) => {
      if this.visit_type_id_intersection_type(ty, itv) {
        let mut intersection_changed = false;
        for &part_ty in itv.parts.iter() {
          traverse_type_id(this, part_ty);
          if get::<IntersectionType>(follow(ty)).is_none() {
            intersection_changed = true;
            break;
          }
        }

        if intersection_changed {
          traverse_type_id(this, ty);
        }
      }
    }
    TypeVariant::Lazy(ltv) => {
      // if (TypeId unwrapped = ltv->unwrapped) traverse(unwrapped);
      let unwrapped: TypeId = ltv.unwrapped;
      if !unwrapped.is_null() {
        traverse_type_id(this, unwrapped);
      }

      // Visiting into LazyType that hasn't been unwrapped may necessarily cause infinite expansion, so we don't do that on purpose.
      // Asserting also makes no sense, because the type _will_ happen here, most likely as a property of some ExternType
      // that doesn't need to be expanded.
    }
    TypeVariant::Singleton(stv) => {
      this.visit_type_id_singleton_type(ty, stv);
    }
    TypeVariant::Blocked(btv) => {
      this.visit_type_id_blocked_type(ty, btv);
    }
    TypeVariant::Unknown(utv) => {
      this.visit_type_id_unknown_type(ty, utv);
    }
    TypeVariant::Never(ntv) => {
      this.visit_type_id_never_type(ty, ntv);
    }
    TypeVariant::PendingExpansion(petv) => {
      if this.visit_type_id_pending_expansion_type(ty, petv) {
        for &a in petv.type_arguments.iter() {
          traverse_type_id(this, a);
        }

        for &a in petv.pack_arguments.iter() {
          this.traverse_type_pack_id(a);
        }
      }
    }
    TypeVariant::Negation(ntv) => {
      if this.visit_type_id_negation_type(ty, ntv) {
        traverse_type_id(this, ntv.ty);
      }
    }
    TypeVariant::TypeFunctionInstance(tfit) => {
      // TypeFunctionDepthCounter tfdc{&typeFunctionDepth};
      this.visitor_base().type_function_depth += 1;

      if this.visit_type_id_type_function_instance_type(ty, tfit) {
        for &p in tfit.type_arguments.iter() {
          traverse_type_id(this, p);
        }

        for &p in tfit.pack_arguments.iter() {
          this.traverse_type_pack_id(p);
        }
      }

      this.visitor_base().type_function_depth -= 1;
    }
  }

  this.visitor_base().seen.unsee(ty as *const ());
}

pub fn traverse_type_pack_id<V: GenericTypeVisitorTrait>(this: &mut V, tp: TypePackId) {
  // tp 是 arena 句柄，遍历期间类型图不被本访问者修改（C++ 同契约；
  // VisitType.h:444 的 traverse(TypePackId) 无 RecursionLimiter）。
  // 句柄解引用收口在下方私有 helper，公共方法保持 safe。
  traverse_type_pack_id_in_limiter(this, tp)
}

fn traverse_type_pack_id_in_limiter<V: GenericTypeVisitorTrait>(this: &mut V, tp: TypePackId) {
  // `tp` 是 arena 存活的 `TypePack` 句柄（C++ 同契约，收口于
  // `type_pack_variant_of`），遍历期间 pack 图不被本访问者改写；
  // `seen` 仅做去重记账，与 `this` 状态借用不别名。
  if this.visitor_base().seen.has_seen(tp as *const ()) {
    this.cycle_type_pack_id(tp);
    return;
  }

  match type_pack_variant_of(tp) {
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
    // ErrorTypePack 即 Unifiable::Error<TypePackId>，variant 携带真实
    // index/synthetic；对齐 C++ get<ErrorTypePack>(tp) 直接传入。
    TypePackVariant::Error(etv) => {
      this.visit_type_pack_id_error_type_pack(tp, etv);
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

  this.visitor_base().seen.unsee(tp as *const ());
}
