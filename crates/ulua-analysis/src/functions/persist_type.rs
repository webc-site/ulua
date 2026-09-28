use alloc::collections::VecDeque;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::table_state::TableState,
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack, get_type,
    get_type_pack,
  },
  records::{
    any_type::AnyType, extern_type::ExternType, free_type::FreeType, function_type::FunctionType,
    generic_type::GenericType, generic_type_pack::GenericTypePack,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, primitive_type::PrimitiveType, singleton_type::SingletonType,
    table_type::TableType, type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    union_type::UnionType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{bound_type::BoundType, type_id::TypeId, type_pack_id::TypePackId},
};

pub fn persist(ty: TypeId) {
  let mut queue: VecDeque<TypeId> = VecDeque::new();
  queue.push_back(ty);

  while let Some(t) = queue.pop_front() {
    // SAFETY: t 指向类型 arena 中的 Type；C++ Type::persistent 原地置位
    unsafe {
      if (*t).persistent {
        continue;
      }

      (*as_mutable_type_id(t)).persistent = true;
    }

    if let Some(btv) = get_type::get::<BoundType>(t) {
      queue.push_back(btv.bound_to);
      continue;
    }

    if let Some(ftv) = get_type::get::<FunctionType>(t) {
      // Safety: arg_types/ret_types 是 t 节点内存储的 TypePackId 句柄，指向
      // 同一类型 arena 存活 TypePackVar（bump 分配地址稳定）；persist_pack 的
      // 判空前置由 arena 不变量满足，且此处共享快照 ftv 仅按值读裸句柄。
      unsafe { persist_pack(ftv.arg_types) };
      // Safety: 同 arg_types 侧——ret_types 为存活 arena 节点句柄。
      unsafe { persist_pack(ftv.ret_types) };
      continue;
    }

    if let Some(ttv) = get_type::get::<TableType>(t) {
      LUAU_ASSERT!(ttv.state != TableState::Free && ttv.state != TableState::Unsealed);

      for prop in ttv.props.values() {
        if let Some(read_ty) = prop.read_ty {
          queue.push_back(read_ty);
        }
        if let Some(write_ty) = prop.write_ty {
          queue.push_back(write_ty);
        }
      }

      if let Some(indexer) = &ttv.indexer {
        queue.push_back(indexer.index_type);
        queue.push_back(indexer.index_result_type);
      }
      continue;
    }

    if let Some(etv) = get_type::get::<ExternType>(t) {
      for prop in etv.props.values() {
        if let Some(read_ty) = prop.read_ty {
          queue.push_back(read_ty);
        }
        if let Some(write_ty) = prop.write_ty {
          queue.push_back(write_ty);
        }
      }
      continue;
    }

    if let Some(utv) = get_type::get::<UnionType>(t) {
      for &opt in &utv.options {
        queue.push_back(opt);
      }
      continue;
    }

    if let Some(itv) = get_type::get::<IntersectionType>(t) {
      for &opt in &itv.parts {
        queue.push_back(opt);
      }
      continue;
    }

    if let Some(mtv) = get_type::get::<MetatableType>(t) {
      queue.push_back(mtv.table());
      queue.push_back(mtv.metatable());
      continue;
    }

    if get_type::get::<GenericType>(t).is_some()
      || get_type::get::<AnyType>(t).is_some()
      || get_type::get::<FreeType>(t).is_some()
      || get_type::get::<SingletonType>(t).is_some()
      || get_type::get::<PrimitiveType>(t).is_some()
      || get_type::get::<NegationType>(t).is_some()
    {
      // nothing to enqueue for these alternatives
      continue;
    }

    if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(t) {
      for &ty in tfit.type_arguments.iter() {
        queue.push_back(ty);
      }

      for &tp in tfit.pack_arguments.iter() {
        // Safety: pack_arguments 元素为实例节点构造时登记的 TypePackId 句柄，
        // 指向 arena 存活 TypePackVar（bump 分配地址稳定），满足 persist_pack
        // 的「tp 非空存活」判空前置。
        unsafe { persist_pack(tp) };
      }
      continue;
    }

    LUAU_ASSERT!(false /* "TypeId is not supported in a persist call" */);
  }
}

/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
pub unsafe fn persist_pack(tp: TypePackId) {
  // SAFETY: tp 指向类型 arena 中的 TypePack；C++ TypePack::persistent 原地置位
  unsafe {
    if (*tp).persistent {
      return;
    }

    (*as_mutable_type_pack(tp)).persistent = true;
  }

  if let Some(p) = get_type_pack::get::<TypePack>(tp) {
    for &ty in &p.head {
      persist(ty);
    }
    if let Some(tail) = p.tail {
      // Safety: tail 是 TypePack 节点内登记的后续 TypePackId 句柄，同 arena
      // 存活、地址稳定；Option::Some 取值在判空语义之外再加一层非空保证。
      unsafe { persist_pack(tail) };
    }
    return;
  }

  if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tp) {
    persist(vtp.ty);
    return;
  }

  if get_type_pack::get::<GenericTypePack>(tp).is_some() {
    return;
  }

  if let Some(tfitp) = get_type_pack::get::<TypeFunctionInstanceTypePack>(tp) {
    for &ty in tfitp.type_arguments.iter() {
      persist(ty);
    }

    for &tp in tfitp.pack_arguments.iter() {
      // Safety: 与 persist() 内 TypeFunctionInstanceType 分支同理——元素是
      // 实例节点登记的存活 TypePackId 句柄，同 arena 地址稳定，满足
      // persist_pack 的判空前置。
      unsafe { persist_pack(tp) };
    }
    return;
  }

  LUAU_ASSERT!(
    false /* "TypePackId is not supported in a persist call" */
  );
}
