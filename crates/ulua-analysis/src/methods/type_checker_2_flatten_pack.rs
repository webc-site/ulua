//! Source: `Analysis/src/TypeChecker2.cpp:2723-2749` (hand-ported)

use alloc::string::String;
use core::ptr::null_mut;

use ulua_ast::records::location::Location;

use crate::{
  enums::polarity::Polarity,
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, finite::finite, first::first, follow_type_pack,
    fresh_index::fresh_index, get_type_pack::get, size_type_pack::size,
  },
  records::{
    arena_id::ArenaId, free_type_pack::FreeTypePack, internal_error::InternalError,
    type_checker_2::TypeChecker2, type_level::TypeLevel, type_pack::TypePack,
    type_pack_var::TypePackVar,
  },
  type_aliases::{
    error_type_pack::ErrorTypePack, type_error_data::IntoTypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
  },
};
impl TypeChecker2 {
  /// C++ `TypeId TypeChecker2::flattenPack(TypePackId pack)`.
  pub(crate) fn flatten_pack(&mut self, pack: TypePackId) -> TypeId {
    let pack = follow_type_pack::follow(pack);

    if let Some(fst) = first(pack, /*ignoreHiddenVariadics*/ false) {
      return fst;
    }

    if let Some(ftp) = get::<FreeTypePack>(pack) {
      let scope = ftp.scope;
      // Safety: self.module 与 self.builtin_types.as_ptr() 均为构造时接线的非空裸指针字段，比
      // self 长寿；fresh_* 经 &mut 写模块 internal_types arena，与 builtin_types 的只读
      // 引用指向不同对象、内存不重叠，单线程无并发别名。
      let result = unsafe {
        (*self.module)
          .internal_types
          .fresh_type_not_null_builtin_types_scope(self.builtin_types.get(), scope)
      };
      // Safety: 同上 self.module 接线非空存活；add_type_pack_type_pack_var 向模块 arena
      // 追加类型包，单线程串行下独占可变借用无别名。
      let free_tail = unsafe {
        (*self.module)
          .internal_types
          .add_type_pack_type_pack_var(TypePackVar {
            ty: TypePackVariant::Free(FreeTypePack {
              index: fresh_index(),
              level: TypeLevel::default(),
              scope,
              polarity: Polarity::Unknown,
            }),
            persistent: false,
            owning_arena: ArenaId::NONE,
          })
      };

      // Safety: pack 是 follow_type_pack_id 后的存活非空 TypePackId（上方 first()/get()
      // 已成功只读解引用），指向 arena 中块地址稳定的 TypePackVar；此刻独占写其 ty 字段，
      // 单线程无其他活跃借用。
      let result_pack = unsafe { &mut *as_mutable_type_pack(pack) };
      result_pack.ty =
        TypePackVariant::TypePack(TypePack::new(alloc::vec![result], Some(free_tail)));

      return result;
    }

    if get::<ErrorTypePack>(pack).is_some() {
      return self.builtin_types_ref().error_type;
    }

    if unsafe {
      // Safety: pack 为 follow 后存活非空 TypePackId；null log 对应 cpp 默认形参
      // `TxnLog* log = nullptr`，finite 内部以 `!log.is_null()` 守卫，不解引用空 log。
      finite(pack, null_mut())
    } && size(pack, None) == 0
    {
      // `(f())` where `f()` returns no values is coerced into `nil`
      return self.builtin_types_ref().nil_type;
    }

    let err = InternalError::new(String::from("flattenPack got a weird pack!"));
    self.report_error_type_error_data_location(err.into_type_error_data(), &Location::default());
    self.builtin_types_ref().error_type
  }
}
