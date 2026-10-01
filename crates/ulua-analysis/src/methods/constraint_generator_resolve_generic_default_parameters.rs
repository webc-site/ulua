use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack,
    emplace_type_pack::emplace_type_pack, shared_mut::shared_mut,
  },
  methods::unifiable::unifiable_bound_type_id_emplace_type_bound_type,
  records::{
    arena_handle::{self, alias_ref},
    constraint_generator::ConstraintGenerator,
    type_fun::TypeFun,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_variant::TypePackVariant},
};
impl ConstraintGenerator {
  /// C++ `ConstraintGenerator::resolveGenericDefaultParameters(Scope* defnScope,
  /// AstStatTypeAlias* alias, const TypeFun& fun)`。形参链已引用化：`defn_scope`
  /// 为本调用内独占可写的存活 `ScopePtr`（写入其 `private_type_bindings` /
  /// `private_type_pack_bindings`，pass 单线程无并存借用）；`alias` 为分析会话
  /// arena 内存活、RTTI 已验证的 `AstStatTypeAlias` 共享借用；`fun` 的
  /// `type_params`/`type_pack_params` 长度须与 `alias.generics`/`alias.generic_packs`
  /// 一致（下方 `LUAU_ASSERT!` 亦校验）；各 `param.default_value` 若为 `Some`，
  /// 须是 arena 内为该泛型形参新建、可独占改写的类型/类型组槽位。
  pub fn resolve_generic_default_parameters(
    &mut self,
    defn_scope: &ScopePtr,
    alias: &AstStatTypeAlias,
    fun: &TypeFun,
  ) {
    LUAU_ASSERT!(alias.generics.size == fun.type_params().len());

    // 形参数组与 TypeFun 形参表一一对应（上方断言担保），zip 走双序，无需下标。
    for (ast_ty_ptr, param) in alias.generics.as_slice().iter().zip(fun.type_params()) {
      let ast_ty = alias_ref(*ast_ty_ptr);

      // `ast_ty.default_value`（Option 表达可空）为 parser 写入的存活 AstType
      // 槽位；命中即解析默认类型并解除 `to_unblock` 槽位的阻塞。
      if let Some(default_value) = ast_ty.default_value
        && let Some(to_unblock) = param.default_value
      {
        let mut resolved = self.resolve_type(
          defn_scope,
          alias_ref(default_value.as_ptr()),
          /* in_type_arguments */ false,
          /* replace_error_with_fresh */ false,
          Polarity::Positive,
        );
        // `to_unblock` 为 arena 内该形参新建、可独占改写的类型槽；
        // `as_mutable_type_id` 得到其 `*mut Type`（既有 const_cast 门面，
        // 单线程 pass 内该改写无并存借用）。
        unifiable_bound_type_id_emplace_type_bound_type(
          arena_handle::alias(as_mutable_type_id(to_unblock)),
          &mut resolved,
        );
      }

      // 向 `defn_scope` 私有绑定表插入 TypeFun::type_fun_type_id；
      // 写窗口经 alias 收口即时物化、即时结束（与 resolve_* 的借用串行）。
      let name_key = ast_ty.name.as_str_or_empty().to_string();
      arena_handle::alias(shared_mut(defn_scope))
        .private_type_bindings
        .insert(name_key, TypeFun::type_fun_type_id(param.ty));
    }

    LUAU_ASSERT!(alias.generic_packs.size == fun.type_pack_params().len());

    for (ast_pack_ptr, param) in alias
      .generic_packs
      .as_slice()
      .iter()
      .zip(fun.type_pack_params())
    {
      let ast_pack = alias_ref(*ast_pack_ptr);

      if let Some(default_value) = ast_pack.default_value
        && let Some(to_unblock) = param.default_value
      {
        let resolved = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
          defn_scope,
          alias_ref(default_value.as_ptr()),
          /* in_type_arguments */ false,
          /* replace_error_with_fresh */ false,
          Polarity::Positive,
        );
        // Safety: `to_unblock` 为 arena 内该形参新建、可独占改写的 TypePackId，
        // `as_mutable_type_pack` 得 `*mut TypePackVar` 满足 emplace 的非空有效入参
        // （既有 const_cast 门面，单线程 pass 内该改写无并存借用）；`resolved` 是
        // 上一步解析出的存活 TypePackId。
        unsafe {
          emplace_type_pack(
            as_mutable_type_pack(to_unblock),
            TypePackVariant::Bound(resolved),
          )
        };
      }

      let name_key = ast_pack.name.as_str_or_empty().to_string();
      arena_handle::alias(shared_mut(defn_scope))
        .private_type_pack_bindings
        .insert(name_key, param.tp);
    }
  }
}
