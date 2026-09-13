use alloc::collections::VecDeque;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::table_state::TableState,
  functions::{
    as_mutable_type::as_mutable_type_id, get_type_alt_j::get_type_id,
    persist_type_alt_b::persist as persist_pack,
  },
  records::{
    any_type::AnyType, extern_type::ExternType, free_type::FreeType, function_type::FunctionType,
    generic_type::GenericType, intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, primitive_type::PrimitiveType, singleton_type::SingletonType,
    table_type::TableType, type_function_instance_type::TypeFunctionInstanceType,
    union_type::UnionType,
  },
  type_aliases::{bound_type::BoundType, type_id::TypeId},
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

    if let Some(btv) = get_type_id::<BoundType>(t) {
      queue.push_back(btv.bound_to);
      continue;
    }

    if let Some(ftv) = get_type_id::<FunctionType>(t) {
      unsafe { persist_pack(ftv.arg_types) };
      unsafe { persist_pack(ftv.ret_types) };
      continue;
    }

    if let Some(ttv) = get_type_id::<TableType>(t) {
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

    if let Some(etv) = get_type_id::<ExternType>(t) {
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

    if let Some(utv) = get_type_id::<UnionType>(t) {
      for &opt in &utv.options {
        queue.push_back(opt);
      }
      continue;
    }

    if let Some(itv) = get_type_id::<IntersectionType>(t) {
      for &opt in &itv.parts {
        queue.push_back(opt);
      }
      continue;
    }

    if let Some(mtv) = get_type_id::<MetatableType>(t) {
      queue.push_back(mtv.table());
      queue.push_back(mtv.metatable());
      continue;
    }

    if get_type_id::<GenericType>(t).is_some()
      || get_type_id::<AnyType>(t).is_some()
      || get_type_id::<FreeType>(t).is_some()
      || get_type_id::<SingletonType>(t).is_some()
      || get_type_id::<PrimitiveType>(t).is_some()
      || get_type_id::<NegationType>(t).is_some()
    {
      // nothing to enqueue for these alternatives
      continue;
    }

    if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(t) {
      for &ty in tfit.type_arguments.iter() {
        queue.push_back(ty);
      }

      for &tp in tfit.pack_arguments.iter() {
        unsafe { persist_pack(tp) };
      }
      continue;
    }

    LUAU_ASSERT!(false /* "TypeId is not supported in a persist call" */);
  }
}
