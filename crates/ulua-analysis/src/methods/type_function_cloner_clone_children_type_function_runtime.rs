use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_mutable_type_function_runtime::{
    get_mutable_type_function_type_id, get_mutable_type_function_type_pack_id,
  },
  records::{
    type_function_any_type::TypeFunctionAnyType, type_function_cloner::TypeFunctionCloner,
    type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_property::TypeFunctionProperty,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_indexer::TypeFunctionTableIndexer,
    type_function_table_type::TypeFunctionTableType, type_function_type_pack::TypeFunctionTypePack,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::{
    type_function_kind::TypeFunctionKind, type_function_type_id::TypeFunctionTypeId,
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
    type_function_type_variant::TypeFunctionTypeVariantMember,
  },
};

/// 源侧（只读）变体负载：cpp `getMutable<T>(ty)` 的 const 投影，变体不符得
/// `None`（与原先「裸指针判空」同义）。arena 解引用与 const_cast 全部收口在
/// `get_mutable_type_function_type_id` 内，此处只把裸指针折成引用。
#[inline]
fn source_of<T: TypeFunctionTypeVariantMember>(ty: TypeFunctionTypeId) -> Option<&'static T> {
  // SAFETY: `ty` 出自 cloner 队列，即 `TypeFunctionRuntime::type_arena`
  // （chunked bump arena，元素一经分配地址不再搬移）内存活节点的 const 句柄；
  // helper 只做一次 tag 探测，命中时返回指向该槽位内变体字段的指针，与节点同
  // 寿命。克隆游程由 `&mut self` 单线程串行驱动，本步骤内源节点不再有其他写
  // 句柄，故由此派生的共享引用可安全延长至游程结束。
  unsafe { get_mutable_type_function_type_id::<T>(ty).as_ref() }
}

/// 目标侧（可写）变体负载：`tfti` 恒为 `shallow_clone_*` 新建的克隆节点。
#[inline]
fn target_of<T: TypeFunctionTypeVariantMember>(tfti: TypeFunctionTypeId) -> Option<&'static mut T> {
  // SAFETY: 同 `source_of` 的 arena 前提；`tfti` 是本轮新建的目标节点（bump
  // 分配，地址必异于源），`run()` 的 `&mut self` 独占推进且 seen 表去重，故本
  // 步骤内该变体字段只有这一个可写句柄。
  unsafe { get_mutable_type_function_type_id::<T>(tfti).as_mut() }
}

/// 类型包侧的 `source_of`，arena 为 `type_pack_arena`，契约同上。
#[inline]
fn pack_source_of<T: TypeFunctionTypePackVariantMember>(
  tp: TypeFunctionTypePackId,
) -> Option<&'static T> {
  // SAFETY: `tp` 为 runtime `type_pack_arena` 存活节点的 const 句柄，地址稳定。
  unsafe { get_mutable_type_function_type_pack_id::<T>(tp).as_ref() }
}

/// 类型包侧的 `target_of`。
#[inline]
fn pack_target_of<T: TypeFunctionTypePackVariantMember>(
  tftp: TypeFunctionTypePackId,
) -> Option<&'static mut T> {
  // SAFETY: `tftp` 为 `shallow_clone_type_function_type_pack_id` 新建的目标
  // 节点，本队列步骤内独占可写。
  unsafe { get_mutable_type_function_type_pack_id::<T>(tftp).as_mut() }
}

impl TypeFunctionCloner {
  /// cpp `cloneChildren(TypeFunctionTypeId ty, TypeFunctionTypeId tfti)`
  /// （`TypeFunctionRuntime.cpp:2742-2770`）：源与目标按同一变体配对后逐臂下发。
  pub fn clone_children_type_id(&mut self, ty: TypeFunctionTypeId, tfti: TypeFunctionTypeId) {
    if let (Some(p1), Some(p2)) = (
      source_of::<TypeFunctionPrimitiveType>(ty),
      target_of::<TypeFunctionPrimitiveType>(tfti),
    ) {
      self.clone_children_primitive_type(p1, p2);
    } else if let (Some(u1), Some(u2)) = (
      source_of::<TypeFunctionUnknownType>(ty),
      target_of::<TypeFunctionUnknownType>(tfti),
    ) {
      self.clone_children_unknown_type(u1, u2);
    } else if let (Some(n1), Some(n2)) = (
      source_of::<TypeFunctionNeverType>(ty),
      target_of::<TypeFunctionNeverType>(tfti),
    ) {
      self.clone_children_never_type(n1, n2);
    } else if let (Some(a1), Some(a2)) = (
      source_of::<TypeFunctionAnyType>(ty),
      target_of::<TypeFunctionAnyType>(tfti),
    ) {
      self.clone_children_any_type(a1, a2);
    } else if let (Some(s1), Some(s2)) = (
      source_of::<TypeFunctionSingletonType>(ty),
      target_of::<TypeFunctionSingletonType>(tfti),
    ) {
      self.clone_children_singleton_type(s1, s2);
    } else if let (Some(u1), Some(u2)) = (
      source_of::<TypeFunctionUnionType>(ty),
      target_of::<TypeFunctionUnionType>(tfti),
    ) {
      self.clone_children_union_type(u1, u2);
    } else if let (Some(i1), Some(i2)) = (
      source_of::<TypeFunctionIntersectionType>(ty),
      target_of::<TypeFunctionIntersectionType>(tfti),
    ) {
      self.clone_children_intersection_type(i1, i2);
    } else if let (Some(n1), Some(n2)) = (
      source_of::<TypeFunctionNegationType>(ty),
      target_of::<TypeFunctionNegationType>(tfti),
    ) {
      self.clone_children_negation_type(n1, n2);
    } else if let (Some(t1), Some(t2)) = (
      source_of::<TypeFunctionTableType>(ty),
      target_of::<TypeFunctionTableType>(tfti),
    ) {
      self.clone_children_table_type(t1, t2);
    } else if let (Some(f1), Some(f2)) = (
      source_of::<TypeFunctionFunctionType>(ty),
      target_of::<TypeFunctionFunctionType>(tfti),
    ) {
      self.clone_children_function_type(f1, f2);
    } else if let (Some(c1), Some(c2)) = (
      source_of::<TypeFunctionExternType>(ty),
      target_of::<TypeFunctionExternType>(tfti),
    ) {
      self.clone_children_extern_type(c1, c2);
    } else if let (Some(g1), Some(g2)) = (
      source_of::<TypeFunctionGenericType>(ty),
      target_of::<TypeFunctionGenericType>(tfti),
    ) {
      self.clone_children_generic_type(g1, g2);
    } else {
      // cpp: LUAU_ASSERT(!"Unknown pair?")，两个句柄必表示同一变体
      LUAU_ASSERT!(false);
    }
  }

  /// cpp `cloneChildren(TypeFunctionTypePackId tp, TypeFunctionTypePackId tftp)`
  /// （`TypeFunctionRuntime.cpp:2772-2784`）。
  pub fn clone_children_type_pack_id(
    &mut self,
    tp: TypeFunctionTypePackId,
    tftp: TypeFunctionTypePackId,
  ) {
    if let (Some(t1), Some(t2)) = (
      pack_source_of::<TypeFunctionTypePack>(tp),
      pack_target_of::<TypeFunctionTypePack>(tftp),
    ) {
      self.clone_children_type_pack(t1, t2);
    } else if let (Some(v1), Some(v2)) = (
      pack_source_of::<TypeFunctionVariadicTypePack>(tp),
      pack_target_of::<TypeFunctionVariadicTypePack>(tftp),
    ) {
      self.clone_children_variadic_type_pack(v1, v2);
    } else if let (Some(g1), Some(g2)) = (
      pack_source_of::<TypeFunctionGenericTypePack>(tp),
      pack_target_of::<TypeFunctionGenericTypePack>(tftp),
    ) {
      self.clone_children_generic_type_pack(g1, g2);
    } else {
      // cpp: LUAU_ASSERT(!"Unknown pair?")
      LUAU_ASSERT!(false);
    }
  }

  /// cpp `cloneChildren(TypeFunctionKind kind, TypeFunctionKind tfkind)`。
  pub fn clone_children_kind(&mut self, kind: &TypeFunctionKind, tfkind: &TypeFunctionKind) {
    if let Some(ty) = TypeFunctionKind::get_if::<TypeFunctionTypeId>(kind)
      && let Some(tfty) = TypeFunctionKind::get_if::<TypeFunctionTypeId>(tfkind)
    {
      self.clone_children_type_id(*ty, *tfty);
      return;
    }

    if let Some(tp) = TypeFunctionKind::get_if::<TypeFunctionTypePackId>(kind)
      && let Some(tftp) = TypeFunctionKind::get_if::<TypeFunctionTypePackId>(tfkind)
    {
      self.clone_children_type_pack_id(*tp, *tftp);
      return;
    }

    LUAU_ASSERT!(false);
  }

  pub fn clone_children_primitive_type(
    &mut self,
    _source: &TypeFunctionPrimitiveType,
    _target: &mut TypeFunctionPrimitiveType,
  ) {
    // noop.
  }

  pub fn clone_children_unknown_type(
    &mut self,
    _source: &TypeFunctionUnknownType,
    _target: &mut TypeFunctionUnknownType,
  ) {
    // noop.
  }

  pub fn clone_children_never_type(
    &mut self,
    _source: &TypeFunctionNeverType,
    _target: &mut TypeFunctionNeverType,
  ) {
    // noop.
  }

  pub fn clone_children_any_type(
    &mut self,
    _source: &TypeFunctionAnyType,
    _target: &mut TypeFunctionAnyType,
  ) {
    // noop.
  }

  pub fn clone_children_singleton_type(
    &mut self,
    _source: &TypeFunctionSingletonType,
    _target: &mut TypeFunctionSingletonType,
  ) {
    // noop.
  }

  /// cpp `cloneChildren(TypeFunctionUnionType* u1, TypeFunctionUnionType* u2)`：
  /// 源只读、目标可写，二者为 arena 中不同节点。
  pub fn clone_children_union_type(
    &mut self,
    source: &TypeFunctionUnionType,
    target: &mut TypeFunctionUnionType,
  ) {
    for ty in &source.components {
      target
        .components
        .push(self.shallow_clone_type_function_type_id(*ty));
    }
  }

  pub fn clone_children_intersection_type(
    &mut self,
    source: &TypeFunctionIntersectionType,
    target: &mut TypeFunctionIntersectionType,
  ) {
    for ty in &source.components {
      target
        .components
        .push(self.shallow_clone_type_function_type_id(*ty));
    }
  }

  pub fn clone_children_negation_type(
    &mut self,
    source: &TypeFunctionNegationType,
    target: &mut TypeFunctionNegationType,
  ) {
    let source_type = source.type_id;
    target.type_id = self.shallow_clone_type_function_type_id(source_type);
  }

  /// cpp `cloneChildren(TypeFunctionTableType* t1, TypeFunctionTableType* t2)`：
  /// 遍历源 props 写入目标（两节点同 arena 但地址互异，源侧全程只读）。
  pub fn clone_children_table_type(
    &mut self,
    source: &TypeFunctionTableType,
    target: &mut TypeFunctionTableType,
  ) {
    for (k, p) in &source.props {
      // std::optional<TypeFunctionTypeId> readTy;
      // if (p.readTy) readTy = shallowClone(*p.readTy);
      let read_ty = p
        .read_ty
        .map(|rt| self.shallow_clone_type_function_type_id(rt));
      // std::optional<TypeFunctionTypeId> writeTy;
      // if (p.writeTy) writeTy = shallowClone(*p.writeTy);
      let write_ty = p
        .write_ty
        .map(|wt| self.shallow_clone_type_function_type_id(wt));

      // t2->props[k] = TypeFunctionProperty{readTy, writeTy};
      target
        .props
        .insert(k.clone(), TypeFunctionProperty { read_ty, write_ty });
    }

    // if (t1->indexer.has_value())
    //     t2->indexer = TypeFunctionTableIndexer(shallowClone(t1->indexer->keyType),
    //                                            shallowClone(t1->indexer->valueType));
    if let Some(idx) = &source.indexer {
      let key_type = self.shallow_clone_type_function_type_id(idx.key_type);
      let value_type = self.shallow_clone_type_function_type_id(idx.value_type);
      target.indexer = Some(TypeFunctionTableIndexer {
        key_type,
        value_type,
      });
    }

    // if (t1->metatable.has_value())
    //     t2->metatable = shallowClone(*t1->metatable);
    if let Some(mt) = source.metatable {
      target.metatable = Some(self.shallow_clone_type_function_type_id(mt));
    }
  }

  pub fn clone_children_function_type(
    &mut self,
    source: &TypeFunctionFunctionType,
    target: &mut TypeFunctionFunctionType,
  ) {
    for ty in &source.generics {
      target
        .generics
        .push(self.shallow_clone_type_function_type_id(*ty));
    }

    for tp in &source.generic_packs {
      target
        .generic_packs
        .push(self.shallow_clone_type_function_type_pack_id(*tp));
    }

    target.arg_types = self.shallow_clone_type_function_type_pack_id(source.arg_types);
    target.ret_types = self.shallow_clone_type_function_type_pack_id(source.ret_types);
    target.arg_names = source.arg_names.clone();
  }

  pub fn clone_children_extern_type(
    &mut self,
    _source: &TypeFunctionExternType,
    _target: &mut TypeFunctionExternType,
  ) {
    // noop.
  }

  pub fn clone_children_generic_type(
    &mut self,
    _source: &TypeFunctionGenericType,
    _target: &mut TypeFunctionGenericType,
  ) {
    // noop.
  }

  pub fn clone_children_type_pack(
    &mut self,
    source: &TypeFunctionTypePack,
    target: &mut TypeFunctionTypePack,
  ) {
    for ty in &source.head {
      target
        .head
        .push(self.shallow_clone_type_function_type_id(*ty));
    }

    if let Some(t) = source.tail {
      let cloned_tail = self.shallow_clone_type_function_type_pack_id(t);
      target.tail = Some(cloned_tail);
    }
  }

  pub fn clone_children_variadic_type_pack(
    &mut self,
    source: &TypeFunctionVariadicTypePack,
    target: &mut TypeFunctionVariadicTypePack,
  ) {
    let source_type = source.type_id;
    target.type_id = self.shallow_clone_type_function_type_id(source_type);
  }

  pub fn clone_children_generic_type_pack(
    &mut self,
    _source: &TypeFunctionGenericTypePack,
    _target: &mut TypeFunctionGenericTypePack,
  ) {
    // noop.
  }
}
