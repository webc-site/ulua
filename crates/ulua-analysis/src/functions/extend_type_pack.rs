use alloc::vec::Vec;

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack_id, follow_type_pack::follow_type_pack_id,
    get_mutable_type_pack::get_mutable_type_pack_id, get_type_pack::get_type_pack_id,
    track_interior_free_type::track_interior_free_type,
    track_interior_free_type_pack::track_interior_free_type_pack,
  },
  records::{
    builtin_types::BuiltinTypes, free_type::FreeType, free_type_pack::FreeTypePack,
    type_arena::TypeArena, type_level::TypeLevel, type_pack::TypePack,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn extend_type_pack(
  arena: &mut TypeArena,
  builtin_types: *mut BuiltinTypes,
  pack: TypePackId,
  length: usize,
  overrides: Vec<Option<TypeId>>,
) -> TypePack {
  let mut result = TypePack {
    head: Vec::new(),
    tail: None,
  };

  let mut current_pack = pack;

  loop {
    current_pack = unsafe { follow_type_pack_id(current_pack) };

    if let Some(p) = get_type_pack_id::<TypePack>(current_pack) {
      let mut i = 0;
      while i < p.head.len() && result.head.len() < length {
        result.head.push(p.head[i]);
        i += 1;
      }

      if result.head.len() == length {
        if i == p.head.len() {
          result.tail = p.tail;
        } else {
          let new_tail = arena.add_type_pack_t(TypePack {
            head: p.head[i..].to_vec(),
            tail: p.tail,
          });
          result.tail = Some(new_tail);
        }
        return result;
      } else if let Some(tail) = p.tail {
        current_pack = tail;
        continue;
      } else {
        return result;
      }
    } else if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(current_pack) {
      while result.head.len() < length {
        result.head.push(vtp.ty);
      }
      result.tail = Some(current_pack);
      return result;
    } else if let Some(ftp) = get_mutable_type_pack_id::<FreeTypePack>(current_pack) {
      let new_pack_scope = ftp.scope;
      let new_pack_polarity = ftp.polarity;

      let mut new_pack = TypePack {
        head: Vec::new(),
        tail: Some(arena.fresh_type_pack(new_pack_scope, new_pack_polarity)),
      };

      track_interior_free_type_pack(new_pack_scope, new_pack.tail.unwrap());

      result.tail = new_pack.tail;

      let mut overrides_index = 0;
      while result.head.len() < length {
        let t = if overrides_index < overrides.len() && overrides[overrides_index].is_some() {
          overrides[overrides_index].unwrap()
        } else {
          let ft = FreeType {
            index: 0,
            level: TypeLevel::default(),
            scope: new_pack_scope,
            forwarded_type_alias: false,
            // SAFETY: builtin_types 由调用方契约保证有效（C++ 同款裸解引用）
            lower_bound: unsafe { (*builtin_types).never_type },
            upper_bound: unsafe { (*builtin_types).unknown_type },
            polarity: new_pack_polarity,
          };
          let new_ty = arena.add_type(ft);
          track_interior_free_type(new_pack_scope, new_ty);
          new_ty
        };

        new_pack.head.push(t);
        result.head.push(*new_pack.head.last().unwrap());
        overrides_index += 1;
      }

      // SAFETY: 原地改写 TypePack 变体，C++ const_cast 语义
      unsafe {
        (*as_mutable_type_pack_id(current_pack)).ty = TypePackVariant::TypePack(new_pack);
      }

      return result;
    } else if get_type_pack_id::<ErrorTypePack>(current_pack).is_some() {
      while result.head.len() < length {
        result.head.push(unsafe { (*builtin_types).error_type });
      }
      result.tail = Some(current_pack);
      return result;
    } else {
      result.tail = Some(current_pack);
      return result;
    }
  }
}
