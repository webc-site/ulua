use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  functions::{fresh_index::fresh_index, get_type, get_type_pack},
  records::{
    arena_id::ArenaId, demoter::Demoter, free_type::FreeType, free_type_pack::FreeTypePack,
    type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant},
};

impl Demoter {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    let ftv = get_type::get::<FreeType>(ty);
    LUAU_ASSERT!(ftv.is_some());
    let demoted_level = self.demoted_level(
      ftv
        .expect("C++ `LUAU_ASSERT(ftv)` 紧邻断言蕴含必命中")
        .level,
    );
    // Safety: arena 是 Demoter::new 从宿主（TypeChecker/normalizer）会话级
    // TypeArena 接线的非空指针，TypedAllocator bump 块地址不移动、比 self 长寿；
    // clean_type_id 由 Tarjan 单线程串行分派驱动，调用期间无其它在册可变借用，
    // 重建 &mut 仅覆盖本表达式。
    let arena = self.arena.get_mut();
    // Safety: builtins 为 C++ `NotNull<BuiltinTypes>` 形参，构造期接线的非空指针
    // 指向 Frontend 全程持有的全局 BuiltinTypes；fresh_type_... 只共享读取它，
    // 与上面 arena 的可变借用是不同对象，无别名冲突。
    arena.fresh_type_not_null_builtin_types_type_level(self.builtins.get(), demoted_level)
  }

  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let ftp = get_type_pack::get::<FreeTypePack>(tp);
    LUAU_ASSERT!(ftp.is_some());
    let demoted_level = self.demoted_level(
      ftp
        .expect("C++ `LUAU_ASSERT(ftp)` 紧邻断言蕴含必命中")
        .level,
    );
    let ftp_var = FreeTypePack {
      index: fresh_index(),
      level: demoted_level,
      scope: null_mut(),
      polarity: Polarity::Unknown,
    };

    let ty_pack_var = TypePackVar {
      ty: TypePackVariant::Free(ftp_var),
      persistent: false,
      owning_arena: ArenaId::NONE,
    };

    // SAFETY: arena 在 Demoter 存活期内有效。
    self.arena.get_mut().add_type_pack_t(ty_pack_var)
  }
}
