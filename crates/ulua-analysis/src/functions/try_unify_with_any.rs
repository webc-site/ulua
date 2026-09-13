use alloc::vec::Vec;
use std::ptr::eq;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{get_mutable_type::get_mutable_type_id, queue_type_pack::queue_type_pack},
  records::{
    extern_type::ExternType, free_type::FreeType, function_type::FunctionType,
    intersection_type::IntersectionType, metatable_type::MetatableType, table_type::TableType,
    r#type::Type, type_arena::TypeArena, unifier::Unifier, union_type::UnionType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_variant::TypeVariant},
};
pub fn try_unify_with_any(
  queue: &mut Vec<TypeId>,
  state: &mut Unifier,
  seen: &mut DenseHashSet<TypeId>,
  seen_type_packs: &mut DenseHashSet<TypePackId>,
  type_arena: *const TypeArena,
  any_type: TypeId,
  any_type_pack: TypePackId,
) {
  while !queue.is_empty() {
    let ty = unsafe { state.log.follow_type_id(*queue.last().unwrap()) };
    queue.pop();

    // SAFETY: ty 指向 TypeArena 内 Type；仅读取 owning_arena 做归属比较。
    if !eq(unsafe { (*ty).owning_arena }, type_arena) {
      continue;
    }

    if seen.find(&ty).is_some() {
      continue;
    }

    seen.insert(ty);

    if get_mutable_type_id::<FreeType>(ty).is_some() {
      state
        .log
        .replace_type_id_t(ty, Type::new(TypeVariant::Bound(any_type)));
    } else if let Some(fun) = get_mutable_type_id::<FunctionType>(ty) {
      queue_type_pack(queue, seen_type_packs, state, fun.arg_types, any_type_pack);
      queue_type_pack(queue, seen_type_packs, state, fun.ret_types, any_type_pack);
    } else if let Some(table) = get_mutable_type_id::<TableType>(ty) {
      for prop in table.props.values() {
        if let Some(prop_ty) = prop.read_ty.or(prop.write_ty) {
          queue.push(prop_ty);
        }
      }

      if let Some(indexer) = &table.indexer {
        queue.push(indexer.index_type);
        queue.push(indexer.index_result_type);
      }
    } else if let Some(mt) = get_mutable_type_id::<MetatableType>(ty) {
      queue.push(mt.table);
      queue.push(mt.metatable);
    } else if get_mutable_type_id::<ExternType>(ty).is_some() {
      // ExternType 没有需要入队的子类型
    } else if let Some(union_) = get_mutable_type_id::<UnionType>(ty) {
      queue.extend(union_.options.iter().copied());
    } else if let Some(intersection) = get_mutable_type_id::<IntersectionType>(ty) {
      queue.extend(intersection.parts.iter().copied());
    }
  }
}
