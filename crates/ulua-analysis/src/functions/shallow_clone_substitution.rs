use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id,
    get_type::{documentation_symbol_of, is_persistent, type_variant_of},
  },
  records::{
    extern_type::ExternType, function_type::FunctionType, intersection_type::IntersectionType,
    metatable_type::MetatableType, negation_type::NegationType,
    pending_expansion_type::PendingExpansionType, table_type::TableType, txn_log::TxnLog,
    r#type::Type, type_arena::TypeArena, type_function_instance_type::TypeFunctionInstanceType,
    unifiable::Error as ErrorType, union_type::UnionType,
  },
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn shallow_clone_type_id_type_arena_txn_log(
  ty: TypeId,
  dest: &mut TypeArena,
  log: *const TxnLog,
) -> TypeId {
  // Safety: `log` 唯一调用方 substitution_clone_substitution 透传其 base.log——该
  // 字段只可能由 substitution_new 写入并 LUAU_ASSERT 非空，取值为 TxnLog::empty()
  // 进程级单例（OnceLock 静态存活）或注入它的会话 TxnLog（寿命覆盖其 substitution）；
  // 此处 follow_type_id 是 log 自有 map 的只读查询，ty 为 arena 存活句柄。
  let mut ty = unsafe { (*log).follow_type_id(ty) };

  // Safety: 同上——log 存活非空；pending_type_id 只读查询，返回 null 或 log 内
  // Box<PendingType> 的稳定堆地址。
  let pty = unsafe { (*log).pending_type_id(ty) };
  if !pty.is_null() {
    // Safety: pty 刚被显式判空；它指向 log.type_var_changes 中 Box 拥有的
    // PendingType（Box 堆地址不随 map 移动），log 按上述前提存活；只取其 pending
    // 字段的节点句柄值（TypeId 本身仍指向 arena 节点）。
    ty = unsafe { &(*pty).pending as *const Type };
  }

  // `ty` 经 follow/pending 解析后仍是 arena 分配的存活 Type 节点——变体与
  // persistent 标志读取收口在 `type_variant_of`/`is_persistent`（同一 arena
  // 节点有效性契约）。`dest` 是调用方给定的独占 &mut 借用，各分支只经它追加
  // 新节点（不移动既有块），与对源节点的读不冲突；返回的恒等分支
  // （Free/Lazy/持久化句柄）原样透出同一存活指针，未产生新别名。
  let res_ty: TypeId = match type_variant_of(ty) {
    // The pointer identities of free and local types is very important.
    // We decline to copy them.
    TypeVariant::Free(_a) => ty,
    TypeVariant::Bound(bound_to) => {
      // This should never happen, but visit() cannot see it.
      LUAU_ASSERT!(false);
      dest.add_tv(Type::new(TypeVariant::Bound(*bound_to)))
    }
    TypeVariant::Generic(a) => dest.add_type(a.clone()),
    TypeVariant::Blocked(a) => dest.add_type(a.clone()),
    TypeVariant::Primitive(_a) => {
      LUAU_ASSERT!(is_persistent(ty));
      ty
    }
    TypeVariant::PendingExpansion(a) => {
      let clone = PendingExpansionType::pending_expansion_type_pending_expansion_type(
        a.prefix,
        a.name,
        a.type_arguments.clone(),
        a.pack_arguments.clone(),
      );
      dest.add_type(clone)
    }
    TypeVariant::Any(_a) => {
      LUAU_ASSERT!(is_persistent(ty));
      ty
    }
    TypeVariant::NoRefine(_a) => {
      LUAU_ASSERT!(is_persistent(ty));
      ty
    }
    TypeVariant::Error(a) => {
      LUAU_ASSERT!(is_persistent(ty) || a.synthetic.is_some());

      if is_persistent(ty) {
        ty
      } else {
        // While this code intentionally works (and clones) even if `a.synthetic`
        // is `std::nullopt`, we still assert above because we consider it a bug to
        // have a non-persistent error type without any associated metadata. We
        // should always use the persistent version in such cases.
        let mut clone = ErrorType::new();
        clone.synthetic = a.synthetic;
        dest.add_tv(Type::new(TypeVariant::Error(clone)))
      }
    }
    TypeVariant::Unknown(_a) => {
      LUAU_ASSERT!(is_persistent(ty));
      ty
    }
    TypeVariant::Never(_a) => {
      LUAU_ASSERT!(is_persistent(ty));
      ty
    }
    TypeVariant::Lazy(_a) => ty,
    TypeVariant::Singleton(a) => dest.add_type(a.clone()),
    TypeVariant::Function(a) => {
      let clone = FunctionType {
        level: a.level,
        arg_types: a.arg_types,
        ret_types: a.ret_types,
        definition: a.definition.clone(),
        has_self: a.has_self,
        generics: a.generics.clone(),
        generic_packs: a.generic_packs.clone(),
        magic: a.magic.clone(),
        tags: a.tags.clone(),
        arg_names: a.arg_names.clone(),
        is_checked_function: a.is_checked_function,
        is_deprecated_function: a.is_deprecated_function,
        deprecated_info: a.deprecated_info.clone(),
        // Not copied by shallowClone; takes its default member initializer.
        has_no_free_or_generic_types: false,
      };
      dest.add_type(clone)
    }
    TypeVariant::Table(a) => {
      LUAU_ASSERT!(a.bound_to.is_none());
      let mut clone =
        TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
          &a.props, a.indexer, a.level, a.scope, a.state,
        );
      clone.definition_module_name = a.definition_module_name.clone();
      clone.definition_location = a.definition_location;
      clone.name = a.name.clone();
      clone.synthetic_name = a.synthetic_name.clone();
      clone.instantiated_type_params = a.instantiated_type_params.clone();
      clone.instantiated_type_pack_params = a.instantiated_type_pack_params.clone();
      clone.tags = a.tags.clone();
      dest.add_type(clone)
    }
    TypeVariant::Metatable(a) => {
      let clone = MetatableType {
        table: a.table,
        metatable: a.metatable,
        synthetic_name: a.synthetic_name.clone(),
      };
      dest.add_type(clone)
    }
    TypeVariant::Union(a) => {
      let clone = UnionType {
        options: a.options.clone(),
      };
      dest.add_type(clone)
    }
    TypeVariant::Intersection(a) => {
      let clone = IntersectionType {
        parts: a.parts.clone(),
      };
      dest.add_type(clone)
    }
    TypeVariant::Extern(a) => {
      let mut clone = ExternType {
        name: a.name.clone(),
        props: a.props.clone(),
        parent: a.parent,
        metatable: a.metatable,
        tags: a.tags.clone(),
        user_data: a.user_data.clone(),
        definition_module_name: a.definition_module_name.clone(),
        definition_location: a.definition_location,
        indexer: a.indexer,
        relation: None,
      };
      if fflag::DebugLuauUserDefinedClasses.get() {
        clone.relation = a.relation.clone();
      }
      dest.add_type(clone)
    }
    TypeVariant::Negation(a) => dest.add_type(NegationType { ty: a.ty }),
    TypeVariant::TypeFunctionInstance(a) => {
      let clone = TypeFunctionInstanceType::new(
        a.function,
        a.type_arguments.clone(),
        a.pack_arguments.clone(),
        a.user_func_name,
        a.user_func_data.clone(),
      );
      dest.add_type(clone)
    }
  };

  if res_ty != ty {
    // Safety: res_ty != ty 保证写目标不是上面按值复用的源节点，而是本函数刚经
    // `dest` 追加的新 arena 节点——handle 未外泄，dest 的 &mut 是其唯一借用者，
    // 重建可变借用无别名；源节点 documentation_symbol 读取收口在
    // `documentation_symbol_of`。
    unsafe {
      (*as_mutable_type_id(res_ty)).documentation_symbol = documentation_symbol_of(ty);
    }
  }

  res_ty
}
