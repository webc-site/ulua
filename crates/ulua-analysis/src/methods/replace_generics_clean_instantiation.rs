use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::table_state::TableState,
  functions::get_mutable_type,
  records::{
    free_type::FreeType, free_type_pack::FreeTypePack, replace_generics::ReplaceGenerics,
    table_type::TableType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ReplaceGenerics {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    LUAU_ASSERT!(self.is_dirty_type_id(ty));

    let log = self.base.base.log;
    let level = self.level;
    let scope = self.scope;
    // Safety: `self.builtin_types` 对应 C++ `NotNull<BuiltinTypes>`，构造期
    // 接线、非空且比本替换器长寿；取共享引用只读 never/unknown 两个内置
    // TypeId，不写穿，单线程串行下无并发可变句柄。函数头一次借用，供
    // 下方各 else 分支共用。
    let builtins = self.builtin_types.get();

    if fflag::LuauReplacerIsSolverAgnostic.get() {
      // Safety: `log` 是入口传入的 `*const TxnLog`（C++ 会话期 TxnLog*），
      // 本次 clean 期间存活；`ty` 为脏标记 TypeId，指向存活 arena Type 节点，
      // 满足 txn_log_get_mutable 入参契约。RTTI class tag 分派，未命中返回
      // null，命中即 repr(C) 基址重合且类型为 TableType。
      let ttv = unsafe { (*log).txn_log_get_mutable::<TableType, TypeId>(ty) };
      if !ttv.is_null() {
        // Safety: 上方 is_null 早退保证 ttv 非空，指向 TxnLog/arena 拥有的
        // 存活 TableType；本共享借用仅存活到下方 clone 读取结束，期间无其他
        // 可变访问。
        let ttv = unsafe { &*ttv };
        let mut clone =
          TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
            &ttv.props,
            ttv.indexer,
            level,
            scope,
            TableState::Free,
          );
        clone.definition_module_name = ttv.definition_module_name.clone();
        clone.definition_location = ttv.definition_location;
        self.base.add_type(clone)
      } else {
        // arena->freshType(builtinTypes, scope, level)
        let free_type = FreeType {
          scope,
          level,
          lower_bound: builtins.never_type,
          upper_bound: builtins.unknown_type,
          ..FreeType::default()
        };
        self.base.add_type(free_type)
      }
    } else {
      // Safety: 同 agnostic 分支——`log`/`ty` 存活有效，class tag 命中才返回
      // 非空 TableType 指针。
      let ttv = unsafe { (*log).txn_log_get_mutable::<TableType, TypeId>(ty) };
      if !ttv.is_null() {
        // Safety: is_null 早退后 ttv 指向存活 TableType，只读借用止于 clone
        // 构造完成，单线程串行无并发可变访问。
        let ttv = unsafe { &*ttv };
        let mut clone =
          TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
            &ttv.props,
            ttv.indexer,
            level,
            scope,
            TableState::Free,
          );
        clone.definition_module_name = ttv.definition_module_name.clone();
        clone.definition_location = ttv.definition_location;
        self.base.add_type(clone)
      } else if fflag::LuauSolverV2.get() {
        let free_type = FreeType {
          scope,
          lower_bound: builtins.never_type,
          upper_bound: builtins.unknown_type,
          ..FreeType::default()
        };
        let res = self.base.add_type(free_type);
        if let Some(ft) = get_mutable_type::get_mutable::<FreeType>(res) {
          ft.level = level;
        }
        res
      } else {
        // arena->freshType(builtinTypes, scope, level)
        let free_type = FreeType {
          scope,
          level,
          lower_bound: builtins.never_type,
          upper_bound: builtins.unknown_type,
          ..FreeType::default()
        };
        self.base.add_type(free_type)
      }
    }
  }

  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    LUAU_ASSERT!(self.is_dirty_type_pack_id(tp));
    let mut pack = FreeTypePack::new(self.level);
    pack.scope = self.scope;
    self.base.add_type_pack(pack)
  }
}
