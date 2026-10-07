//! `demoter` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  functions::{fresh_index::fresh_index, get_type, get_type_pack},
  macros::substitution_vtable,
  records::{
    arena_handle::Handle, arena_id::ArenaId, builtin_types::BuiltinTypes, demoter::Demoter,
    extern_type::ExternType, free_type::FreeType, free_type_pack::FreeTypePack,
    substitution::Substitution, txn_log::TxnLog, type_arena::TypeArena, type_level::TypeLevel,
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

// `Demoter::demote`（TypeInfer.cpp:818-824）。

impl Demoter {
  pub fn demote(&mut self, expected_types: &mut [Option<TypeId>]) {
    self.install_substitution_vtable();
    for slot in expected_types.iter_mut().flatten() {
      // C++ `ty = substitute(*ty)`；`None` 仅在递归限位等异常路径出现，
      // 此时保留原类型（对应 C++ 断言路径外的保守处理）。
      if let Some(demoted) = self.base.substitute_type_id(*slot) {
        *slot = demoted;
      }
    }
  }
}

impl Demoter {
  fn demoted_level(&mut self, level: TypeLevel) -> TypeLevel {
    TypeLevel {
      level: level.level + 5000,
      sub_level: level.sub_level,
    }
  }
}

// `Demoter` 构造 + 虚函数挂接。
//
// C++ `Demoter(TypeArena* arena, NotNull<BuiltinTypes> builtins)`（TypeInfer.cpp:780）。
// `isDirty` / `clean` / `ignoreChildren` 的覆写体在各自的
// `demoter_*` 方法文件中，此处经 `SubstitutionVtable` 装上——与
// `Replacer::install_substitution_vtable` 同款模式。

// C++ 未覆写 `ignoreChildren(TypePackId)`，保持基类默认 false（pack 侧三槽中
// isDirty/clean 仍为真实转发，见 substitution_vtable 模块文档的统一安全论证）。
substitution_vtable!(false_tp, pub(crate) Demoter, ic = ignore_children);
impl Demoter {
  pub fn new(arena: Handle<TypeArena>, builtins: Handle<BuiltinTypes>) -> Self {
    let mut this = Demoter {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      arena,
      builtins,
    };
    this.install_substitution_vtable();
    this
  }
}

impl Demoter {
  pub fn ignore_children(&mut self, ty: TypeId) -> bool {
    let et = get_type::get::<ExternType>(ty);
    et.is_some()
  }
}

impl Demoter {
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    let ftv = get_type::get::<FreeType>(ty);
    ftv.is_some()
  }

  pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let ftp = get_type_pack::get::<FreeTypePack>(tp);
    ftp.is_some()
  }
}
