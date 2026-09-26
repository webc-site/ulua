use alloc::vec::Vec;
use core::iter::{repeat, repeat_n};

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, follow_type_pack, get_mutable_type_pack,
    get_type_pack, track_interior_free_type::track_interior_free_type,
    track_interior_free_type_pack::track_interior_free_type_pack,
  },
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, free_type::FreeType,
    free_type_pack::FreeTypePack, type_arena::TypeArena, type_level::TypeLevel,
    type_pack::TypePack, variadic_type_pack::VariadicTypePack,
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
  builtin_types: Handle<BuiltinTypes>,
  pack: TypePackId,
  length: usize,
  overrides: Vec<Option<TypeId>>,
) -> TypePack {
  let mut result = TypePack::empty();

  let mut current_pack = pack;

  // builtin_types 是调用方（unifier/类型检查器）构造期接线的常驻 BuiltinTypes 句柄，
  // 比本次调用长寿；整程仅读取其 never/unknown/error 三个常量字段，期间无人对该
  // 对象建立可变借用，单线程串行下共享借用无别名。
  let builtin = builtin_types.get();

  loop {
    current_pack = follow_type_pack::follow(current_pack);

    if let Some(p) = get_type_pack::get::<TypePack>(current_pack) {
      // 本次迭代最多再填充的字节数
      let take = length.saturating_sub(result.head.len()).min(p.head.len());
      result.head.extend(p.head.iter().take(take).copied());

      if result.head.len() == length {
        if take == p.head.len() {
          result.tail = p.tail;
        } else {
          let new_tail = arena.add_type_pack_t(TypePack::new(p.head[take..].to_vec(), p.tail));
          result.tail = Some(new_tail);
        }
        return result;
      } else if let Some(tail) = p.tail {
        current_pack = tail;
        continue;
      } else {
        return result;
      }
    } else if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(current_pack) {
      let fill = length.saturating_sub(result.head.len());
      result.head.extend(repeat_n(vtp.ty, fill));
      result.tail = Some(current_pack);
      return result;
    } else if let Some(ftp) = get_mutable_type_pack::get_mutable::<FreeTypePack>(current_pack) {
      let new_pack_scope = ftp.scope;
      let new_pack_polarity = ftp.polarity;

      // 双写合一：fresh 句柄先行绑定，构造与追踪复用同一值，
      // 免去对刚写入 Some 的 `new_pack.tail` 再判空取回。
      let fresh_tail = arena.fresh_type_pack(new_pack_scope, new_pack_polarity);
      let mut new_pack = TypePack::new(Vec::new(), Some(fresh_tail));

      track_interior_free_type_pack(new_pack_scope, fresh_tail);

      result.tail = new_pack.tail;

      // overrides 逐位对齐填充，越界或 None 处生成新的自由类型
      let fill = length.saturating_sub(result.head.len());
      let overrides = overrides.into_iter().chain(repeat(None));
      for t in overrides.take(fill) {
        let t = t.unwrap_or_else(|| {
          let ft = FreeType {
            index: 0,
            level: TypeLevel::default(),
            scope: new_pack_scope,
            forwarded_type_alias: false,
            lower_bound: builtin.never_type,
            upper_bound: builtin.unknown_type,
            polarity: new_pack_polarity,
          };
          let new_ty = arena.add_type(ft);
          track_interior_free_type(new_pack_scope, new_ty);
          new_ty
        });

        new_pack.head.push(t);
        result.head.push(t);
      }

      // SAFETY: 原地改写 TypePack 变体，C++ const_cast 语义
      unsafe {
        (*as_mutable_type_pack(current_pack)).ty = TypePackVariant::TypePack(new_pack);
      }

      return result;
    } else if get_type_pack::get::<ErrorTypePack>(current_pack).is_some() {
      let fill = length.saturating_sub(result.head.len());
      result.head.extend(repeat_n(builtin.error_type, fill));
      result.tail = Some(current_pack);
      return result;
    } else {
      result.tail = Some(current_pack);
      return result;
    }
  }
}
