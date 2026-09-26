use core::ptr::{NonNull, null_mut};

use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack,
    emplace_type_pack::emplace_type_pack,
  },
  methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type,
  records::{constraint_generator::ConstraintGenerator, scope::Scope, type_fun::TypeFun},
  type_aliases::type_pack_variant::TypePackVariant,
};
impl ConstraintGenerator {
  /// # Safety
  /// 直译 cpp `ConstraintGenerator::resolveGenericDefaultParameters`：
  /// - `alias` 必须非空并指向分析会话 arena 内存活、RTTI 已验证的 `AstStatTypeAlias`；
  /// - `defn_scope` 必须非空并指向存活、在本调用内被独占可写的 `Scope`（写入其
  ///   `private_type_bindings` / `private_type_pack_bindings`，且 pass 单线程无并存借用）；
  /// - `fun` 为该 alias 的 `TypeFun`，其 `type_params`/`type_pack_params` 长度须与
  ///   `alias.generics`/`alias.generic_packs` 一致（下方 `LUAU_ASSERT!` 亦校验）；
  /// - 每个 `param.default_value` 若为 `Some`，须是 arena 内为该泛型形参新建、可独占
  ///   改写的类型/类型组槽位。
  pub unsafe fn resolve_generic_default_parameters(
    &mut self,
    defn_scope: *mut Scope,
    alias: *mut AstStatTypeAlias,
    fun: &TypeFun,
  ) {
    // Safety: `alias` 按函数级契约为 arena 内存活非空指针，共享再借用 `&*alias` 仅作
    // 只读遍历，与本次 `&mut self` 状态不相交。
    let alias_ref = unsafe { &*alias };
    LUAU_ASSERT!(alias_ref.generics.size == fun.type_params().len());

    for (i, &ast_ty) in alias_ref.generics.as_slice().iter().enumerate() {
      let param = &fun.type_params()[i];

      // Safety: `ast_ty` 是 alias.generics 数组槽位、parser 写入的非空存活 AstType 指针；
      // 此处仅 Copy 读其 `default_value`（Option 表达可空）作短路条件。
      if unsafe { (*ast_ty).default_value }.is_some()
        && let Some(to_unblock) = param.default_value
      {
        let resolves_to = unsafe { (*ast_ty).default_value }.map_or(null_mut(), NonNull::as_ptr);
        let resolved = self.resolve_type(
          defn_scope,
          resolves_to,
          /* in_type_arguments */ false,
          /* replace_error_with_fresh */ false,
          Polarity::Positive,
        );
        unsafe {
          // Safety: `to_unblock` 按契约为 arena 内为该形参新建、可独占改写的类型槽，
          // `as_mutable_type_id` 得到其 `*mut Type`，`&mut *` 再借用只活过这次 emplace；
          // `resolved` 是本迭代局部的 Copy TypeId。constraint pass 单线程，无并存别名。
          let mut resolved = resolved;
          unifiable_bound_type_id_emplace_type_bound_type(
            &mut *as_mutable_type_id(to_unblock),
            &mut resolved,
          );
        }
      }

      unsafe {
        // Safety: ast_ty 存活非空，`name` 为 Copy 读取；`defn_scope` 按函数级契约为
        // 存活、本调用独占可写的 Scope，向其私有绑定表插入 TypeFun::type_fun_type_id。
        let name_key = (*ast_ty).name.as_str_or_empty().to_string();
        (*defn_scope)
          .private_type_bindings
          .insert(name_key, TypeFun::type_fun_type_id(param.ty));
      }
    }

    LUAU_ASSERT!(alias_ref.generic_packs.size == fun.type_pack_params().len());

    for (i, &ast_pack) in alias_ref.generic_packs.as_slice().iter().enumerate() {
      let param = &fun.type_pack_params()[i];

      // Safety: `ast_pack` 为 alias.generic_packs 数组槽位、parser 写入的非空存活
      // AstTypePack；仅 Copy 读 default_value（Option 表达可空）作短路条件。
      if unsafe { (*ast_pack).default_value }.is_some()
        && let Some(to_unblock) = param.default_value
      {
        let resolves_to = unsafe { (*ast_pack).default_value }.map_or(null_mut(), NonNull::as_ptr);
        let resolved = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
          defn_scope,
          resolves_to,
          /* in_type_arguments */ false,
          /* replace_error_with_fresh */ false,
          Polarity::Positive,
        );
        {
          unsafe {
            // Safety: `to_unblock` 为 arena 内该形参新建、可独占改写的 TypePackId，
            // `as_mutable_type_pack` 得 `*mut TypePackVar` 满足 emplace_type_pack 的
            // 非空有效入参契约；单线程 pass 内该改写无并存借用，resolved 为 Copy 值。
            emplace_type_pack(
              as_mutable_type_pack(to_unblock),
              TypePackVariant::Bound(resolved),
            )
          };
        }
      }

      unsafe {
        // Safety: ast_pack 存活非空，`name` Copy 读取；defn_scope 为存活、本调用独占
        // 可写的 Scope，写其私有类型组绑定表。
        let name_key = (*ast_pack).name.as_str_or_empty().to_string();
        (*defn_scope)
          .private_type_pack_bindings
          .insert(name_key, param.tp);
      }
    }
  }
}
