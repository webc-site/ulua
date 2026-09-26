use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_pack_generic::AstTypePackGeneric,
    ast_type_pack_variadic::AstTypePackVariadic,
  },
  rtti::{AstNodePtr, ast_node_try_as_ptr},
};
use ulua_common::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  functions::{arc_as_mut::arc_as_mut, follow_type_pack, get_mutable_type_pack},
  records::{
    constraint_generator::ConstraintGenerator,
    generic_type_pack::GenericTypePack,
    scope::Scope,
    unknown_symbol::{Context, UnknownSymbol},
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_error_data::IntoTypeErrorData, type_id::TypeId, type_pack_id::TypePackId},
};

impl ConstraintGenerator {
  pub(crate) fn resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
    &mut self,
    scope: *mut Scope,
    tp: *mut AstTypePack,
    in_type_argument: bool,
    replace_error_with_fresh: bool,
    initial_polarity: Polarity,
  ) -> TypePackId {
    let _polarity = initial_polarity;
    // Safety: `scope`/`tp` 原样透传给被调，满足其对存活 Scope（C++ NotNull<Scope>）
    // 与存活 AstTypePack 节点的裸指针契约（C++:4947 resolveTypePack 五参重载直接
    // 转调 resolveTypePack_）。
    unsafe {
      self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
        scope,
        tp,
        in_type_argument,
        replace_error_with_fresh,
      )
    }
  }

  /// # Safety
  /// 对应 C++ `resolveTypePack_(const ScopePtr& scope, AstTypePack* tp,
  /// bool inTypeArgument, bool replaceErrorWithFresh)`（ConstraintGenerator.cpp:4959）
  /// 的裸指针形参前提，逐参数：
  /// * `scope`：调用期存活的 `Scope`（C++ 以 NotNull<Scope>/ScopePtr 传入）；本
  ///   函数解引用其读取 `lookupPack`，且递归透传给 `resolve_type_inner`；
  /// * `tp`：本模块 parse arena 持有的存活 `AstTypePack` 派生节点，其 class
  ///   索引与 `explicit`/`variadic`/`generic` 子字段在整个解包期间有效；约束
  ///   生成阶段无人以他途可变借用该 AST 节点；
  ///
  /// 另要求 `self` 的 arena/builtin_types/module 构造期非空不变量成立。
  pub unsafe fn resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
    &mut self,
    scope: *mut Scope,
    tp: *mut AstTypePack,
    in_type_argument: bool,
    replace_error_with_fresh: bool,
  ) -> TypePackId {
    let result: TypePackId;

    let node = tp.as_ast_node();
    // Safety: AstTypePack 派生类型 repr(C) 首字段为 AstNode，`tp` 按契约存活；
    // ast_node_try_as_ptr 判空并按 class_index 分派，命中即与 `tp` 同址同型，且各臂仅
    // 只读子字段（type_list / variadic_type / generic_name），不写节点。
    if let Some(explicit) = unsafe { ast_node_try_as_ptr::<AstTypePackExplicit>(node) } {
      result = self.resolve_type_pack_scope_ptr_ast_type_list_bool_bool(
        scope,
        &explicit.type_list,
        in_type_argument,
        replace_error_with_fresh,
      );
    } else if let Some(variadic) = unsafe { ast_node_try_as_ptr::<AstTypePackVariadic>(node) } {
      let ty: TypeId = self.resolve_type_inner(
        scope,
        variadic.variadic_type,
        in_type_argument,
        replace_error_with_fresh,
      );
      // Safety: arena 独占追加窗口；ty 为 arena 驻留 TypeId（C++:4969
      // `arena->addTypePack(TypePackVar{VariadicTypePack{ty}})`）。
      result = self
        .arena
        .get_mut()
        .add_type_pack_t(VariadicTypePack { ty, hidden: false });
    } else if let Some(generic) = unsafe { ast_node_try_as_ptr::<AstTypePackGeneric>(node) } {
      let generic_name_str = generic.generic_name.as_str_or_empty().to_string();

      // Safety: `scope` 按本函数契约为存活 Scope，`lookup_pack` 只读其名称
      // 映射表（C++:4975 `scope->lookupPack(gen->genericName.value)`）。
      if let Some(lookup) = unsafe { &*scope }.lookup_pack(&generic_name_str) {
        result = lookup;
      } else {
        let error = UnknownSymbol::new(generic_name_str, Context::Type);
        // Safety: `tp` 存活契约——仅读首字段 location（C++:4983 reportError
        // 同参 `tp->location`）。
        let location = unsafe { (*tp).base.location };
        self.report_error(location, error.into_type_error_data());
        // Safety: 构造期非空 builtin_types 单例的 error_type_pack 常量槽
        // （C++:4984 `builtinTypes->errorTypePack`）。
        result = self.builtin_types.get().error_type_pack;
      }
    } else {
      LUAU_ASSERT!(false);
      // Safety: 不可达兜底臂（tp 必属三种派生之一）。
      result = self.builtin_types.get().error_type_pack;
    }

    // 对照 C++：`result = follow(result); if (auto gtp = getMutable<GenericTypePack>(result))`
    let followed = follow_type_pack::follow(result);
    if let Some(gtp) = get_mutable_type_pack::get_mutable::<GenericTypePack>(followed) {
      gtp.polarity = (gtp.polarity & Polarity::Mixed) | self.polarity;
    }

    if let Some(module) = &self.module {
      let module_ptr = arc_as_mut(module);
      // Safety: module Arc 目标由 self.module 持有与会话同寿，ast_resolved_type_packs
      // 键 `tp` 为存活 AST 指针、值 result 为 arena 驻留 TypePackId
      // （C++:5001 `module->astResolvedTypePacks[tp] = result`）。
      unsafe {
        *(*module_ptr)
          .ast_resolved_type_packs
          .get_or_insert(tp as *const AstTypePack) = result;
      }
    }

    result
  }

  pub(crate) fn resolve_type_pack_scope_ptr_ast_type_list_bool_bool(
    &mut self,
    _scope: *mut Scope,
    _list: &AstTypeList,
    _in_type_arguments: bool,
    _replace_error_with_fresh: bool,
  ) -> TypePackId {
    let mut head = Vec::new();
    for head_ty in _list.types.iter() {
      head.push(self.resolve_type_inner(
        _scope,
        *head_ty,
        _in_type_arguments,
        _replace_error_with_fresh,
      ));
    }
    let tail = if !_list.tail_type.is_null() {
      // Safety: 分支判非空后 `_list.tail_type` 是 list 契约担保的存活 AstTypePack
      // 子节点，`_scope` 原样透传（C++:5017 `list.tailType` 递归 resolveTypePack_）。
      Some(unsafe {
        self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
          _scope,
          _list.tail_type,
          _in_type_arguments,
          _replace_error_with_fresh,
        )
      })
    } else {
      None
    };
    self.add_type_pack(head, tail)
  }

  pub fn resolve_type_pack_scope_ptr_ast_type_list_bool_bool_polarity(
    &mut self,
    scope: *mut Scope,
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
