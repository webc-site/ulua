use alloc::vec::Vec;
use core::ptr::from_ref;

use ulua_ast::{
  records::{
    ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_pack_generic::AstTypePackGeneric,
    ast_type_pack_variadic::AstTypePackVariadic,
  },
  rtti::ast_node_try_as,
};
use ulua_common::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  functions::{follow_type_pack, get_mutable_type_pack, shared_mut::shared_mut},
  records::{
    arena_handle::{alias, alias_opt, alias_ref},
    constraint_generator::ConstraintGenerator,
    generic_type_pack::GenericTypePack,
    unknown_symbol::{Context, UnknownSymbol},
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_error_data::IntoTypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

impl ConstraintGenerator {
  /// C++ `resolveTypePack(const ScopePtr& scope, AstTypePack* tp, bool, bool, Polarity)`
  /// （cpp:4947 五参重载直接转调 resolveTypePack_）。形参链已引用化：`scope` 为
  /// 调用方 `ScopePtr` 共享借用，`tp` 为 parse arena 存活节点共享引用。
  pub(crate) fn resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
    &mut self,
    scope: &ScopePtr,
    tp: &AstTypePack,
    in_type_argument: bool,
    replace_error_with_fresh: bool,
    initial_polarity: Polarity,
  ) -> TypePackId {
    let _polarity = initial_polarity;
    self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
      scope,
      tp,
      in_type_argument,
      replace_error_with_fresh,
    )
  }

  /// C++ `resolveTypePack_(const ScopePtr& scope, AstTypePack* tp, bool, bool)`
  /// （ConstraintGenerator.cpp:4959）。`scope` 为调用期存活的 `ScopePtr` 共享借用，
  /// `tp` 为 parse arena 持有的存活 `AstTypePack` 派生节点（RTTI 判别只读下转，
  /// 各臂仅读子字段）；另要求 `self` 的 arena/builtin_types/module 构造期不变量成立。
  pub(crate) fn resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
    &mut self,
    scope: &ScopePtr,
    tp: &AstTypePack,
    in_type_argument: bool,
    replace_error_with_fresh: bool,
  ) -> TypePackId {
    let result: TypePackId;

    // RTTI 判别下转：class_index 命中即与 `tp` 同址同型，各臂仅只读子字段
    // （type_list / variadic_type / generic_name），不写节点。
    if let Some(explicit) = ast_node_try_as::<AstTypePackExplicit>(tp) {
      result = self.resolve_type_pack_scope_ptr_ast_type_list_bool_bool(
        scope,
        &explicit.type_list,
        in_type_argument,
        replace_error_with_fresh,
      );
    } else if let Some(variadic) = ast_node_try_as::<AstTypePackVariadic>(tp) {
      let ty: TypeId = self.resolve_type_inner(
        scope,
        // variadic_type 槽已句柄化（`...T` 文法必建 T），get() 给出共享引用。
        variadic.variadic_type.get(),
        in_type_argument,
        replace_error_with_fresh,
      );
      // arena 独占追加窗口；ty 为 arena 驻留 TypeId（C++:4969
      // `arena->addTypePack(TypePackVar{VariadicTypePack{ty}})`）。
      result = self
        .arena
        .get_mut()
        .add_type_pack_t(VariadicTypePack { ty, hidden: false });
    } else if let Some(generic) = ast_node_try_as::<AstTypePackGeneric>(tp) {
      let generic_name_str = generic.generic_name.as_str_or_empty().to_string();

      // `scope` 为存活 Scope，`lookup_pack` 只读其名称映射表
      // （C++:4975 `scope->lookupPack(gen->genericName.value)`）。
      if let Some(lookup) = scope.lookup_pack(&generic_name_str) {
        result = lookup;
      } else {
        let error = UnknownSymbol::new(generic_name_str, Context::Type);
        // 仅读首字段 location（C++:4983 reportError 同参 `tp->location`）。
        let location = tp.base.location;
        self.report_error(location, error.into_type_error_data());
        // 构造期非空 builtin_types 单例的 error_type_pack 常量槽
        // （C++:4984 `builtinTypes->errorTypePack`）。
        result = self.builtin_types.get().error_type_pack;
      }
    } else {
      LUAU_ASSERT!(false);
      // 不可达兜底臂（tp 必属三种派生之一）。
      result = self.builtin_types.get().error_type_pack;
    }

    // 对照 C++：`result = follow(result); if (auto gtp = getMutable<GenericTypePack>(result))`
    let followed = follow_type_pack::follow(result);
    if let Some(gtp) = get_mutable_type_pack::get_mutable::<GenericTypePack>(followed) {
      gtp.polarity = (gtp.polarity & Polarity::Mixed) | self.polarity;
    }

    if let Some(module) = &self.module {
      // module Arc 目标由 self.module 持有与会话同寿，ast_resolved_type_packs
      // 键 `tp` 为存活 AST 指针、值 result 为 arena 驻留 TypePackId
      // （C++:5001 `module->astResolvedTypePacks[tp] = result`）；经 alias 收口写。
      let module_ptr = shared_mut(module);
      *alias(module_ptr)
        .ast_resolved_type_packs
        .get_or_insert(from_ref(tp)) = result;
    }

    result
  }

  pub(crate) fn resolve_type_pack_scope_ptr_ast_type_list_bool_bool(
    &mut self,
    scope: &ScopePtr,
    list: &AstTypeList,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
  ) -> TypePackId {
    let mut head = Vec::new();
    for &head_ty in list.types.iter() {
      head.push(self.resolve_type_inner(
        scope,
        alias_ref(head_ty),
        in_type_arguments,
        replace_error_with_fresh,
      ));
    }
    let tail = alias_opt(list.tail_type).map(|tail| {
      // 分支判非空后 `list.tail_type` 是 list 契约担保的存活 AstTypePack
      // 子节点，`scope` 原样透传（C++:5017 `list.tailType` 递归 resolveTypePack_）。
      self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
        scope,
        tail,
        in_type_arguments,
        replace_error_with_fresh,
      )
    });
    self.add_type_pack(head, tail)
  }

  pub fn resolve_type_pack_scope_ptr_ast_type_list_bool_bool_polarity(
    &mut self,
    scope: &ScopePtr,
    list: &AstTypeList,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
    initial_polarity: Polarity,
  ) -> TypePackId {
    let _polarity = initial_polarity;
    self.resolve_type_pack_scope_ptr_ast_type_list_bool_bool(
      scope,
      list,
      in_type_arguments,
      replace_error_with_fresh,
    )
  }
}
