use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::records::{
  ast_array::AstArray, ast_generic_type::AstGenericType, ast_generic_type_pack::AstGenericTypePack,
  ast_node::AstNode, node_handle::Nodes,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  records::{
    arena_handle::{alias, alias_ref},
    duplicate_generic_parameter::DuplicateGenericParameter,
    generic_type::GenericType,
    generic_type_definition::GenericTypeDefinition,
    generic_type_definitions::GenericTypeDefinitions,
    generic_type_pack::GenericTypePack,
    generic_type_pack_definition::GenericTypePackDefinition,
    scope::Scope,
    scope_registry::{resolve_scope, resolve_scope_mut},
    type_checker::TypeChecker,
    type_error::TypeError,
    type_fun::TypeFun,
    type_level::TypeLevel,
    type_pack_var::TypePackVar,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_error_data::TypeErrorData},
};
impl TypeChecker {
  pub fn create_generic_types(
    &mut self,
    scope: &ScopePtr,
    level_opt: Option<TypeLevel>,
    node: &AstNode,
    generic_names: &AstArray<*mut AstGenericType>,
    generic_pack_names: &AstArray<*mut AstGenericTypePack>,
    use_cache: bool,
  ) -> GenericTypeDefinitions {
    // iter_nodes：unsafe 收口在 AstArray::iter_nodes（元素由 parser 成对写入，
    // 恒指向存活节点）。
    self.create_generic_types_impl(
      scope,
      level_opt,
      node,
      generic_names.iter_nodes(),
      generic_pack_names.iter_nodes(),
      use_cache,
    )
  }

  /// `&Nodes` 槽位形态：`AstExprFunction{generics, generic_packs}` 句柄化后,
  /// `Node::get` 直出 arena 存活只读引用,与 [`Self::create_generic_types`] 同义。
  pub fn create_generic_types_nodes<'g>(
    &mut self,
    scope: &ScopePtr,
    level_opt: Option<TypeLevel>,
    node: &AstNode,
    generic_names: &'g Nodes<AstGenericType>,
    generic_pack_names: &'g Nodes<AstGenericTypePack>,
    use_cache: bool,
  ) -> GenericTypeDefinitions {
    self.create_generic_types_impl(
      scope,
      level_opt,
      node,
      generic_names.iter(),
      generic_pack_names.iter(),
      use_cache,
    )
  }

  fn create_generic_types_impl<'g>(
    &mut self,
    scope: &ScopePtr,
    level_opt: Option<TypeLevel>,
    node: &AstNode,
    generic_names: impl Iterator<Item = &'g AstGenericType>,
    generic_pack_names: impl Iterator<Item = &'g AstGenericTypePack>,
    use_cache: bool,
  ) -> GenericTypeDefinitions {
    let scope_ptr = scope.as_ref() as *const Scope as *mut Scope;
    LUAU_ASSERT!(scope.parent.is_some());

    let level = level_opt.unwrap_or(scope.level);

    let mut generics = Vec::new();

    for generic in generic_names {
      // Option::map 直折原「is_some 守卫 + map_or(null_mut(), as_ptr) 倒灌」的
      // C 型哨兵（§2/§3）；alias_ref 仍按 arena 句柄门面解引用。
      let default_value = generic
        .default_value
        .map(|dv| self.resolve_type(scope.clone(), alias_ref(dv.as_ptr().cast_const())));

      let n = generic.name.as_str_or_empty().to_string();

      if scope.private_type_bindings.contains_key(&n)
        || scope.private_type_pack_bindings.contains_key(&n)
      {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          node.location,
          TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(n.clone())),
        ));
      }

      let g = if use_cache {
        // 父 scope 以 ScopeId 句柄横传：只读走 resolve_scope，缓存写回走
        // resolve_scope_mut（scope_registry 写回纪律，替换原经 parent Arc 的裸
        // 指针写，共享读与独占写顺序借用、互不重叠）。
        let parent = scope.parent.expect(
          "函数开头 LUAU_ASSERT!(scope.parent.is_some()) 蕴含 Some，循环内无 parent 重赋值",
        );
        let existing = resolve_scope(parent)
          .expect("parent 句柄出自 register_scope，注册表内恒可解析（契约 1）")
          .type_alias_type_parameters
          .get(&n)
          .copied();

        match existing {
          Some(c) if !c.is_null() => c,
          _ => {
            let new_ty = self.add_type(&GenericType::generic_type_type_level_name(level, &n));
            // 写入镜像 C++ `parentScope->typeAliasTypeParameters[name] = ty`
            // （TypeInfer.cpp createGenericTypes useCache 分支），检查期单线程独占。
            resolve_scope_mut(parent)
              .expect("parent 句柄出自 register_scope，注册表内恒可解析（契约 1）")
              .type_alias_type_parameters
              .insert(n.clone(), new_ty);
            new_ty
          }
        }
      } else {
        self.add_type(&GenericType::generic_type_type_level_name(level, &n))
      };

      generics.push(GenericTypeDefinition {
        ty: g,
        default_value,
      });
      // C++ `scope->privateTypeBindings[n] = ...`。
      alias(scope_ptr)
        .private_type_bindings
        .insert(n, TypeFun::type_fun_type_id(g));
    }

    let mut generic_packs = Vec::new();

    for generic_pack in generic_pack_names {
      // 同类型参数分支：map 直折哨兵倒灌。
      let default_value = generic_pack.default_value.map(|dv| {
        self.resolve_type_pack_scope_ptr_ast_type_pack(
          scope.clone(),
          alias_ref(dv.as_ptr().cast_const()),
        )
      });

      let n = generic_pack.name.as_str_or_empty().to_string();

      if scope.private_type_pack_bindings.contains_key(&n)
        || scope.private_type_bindings.contains_key(&n)
      {
        self.report_error_type_error(&TypeError::type_error_location_type_error_data(
          node.location,
          TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(n.clone())),
        ));
      }

      // 父 scope 句柄化上溯：同类型参数分支，只读 resolve_scope、写回
      // resolve_scope_mut（scope_registry 写回纪律）。
      let parent = scope
        .parent
        .expect("函数开头 LUAU_ASSERT!(scope.parent.is_some()) 蕴含 Some，循环内无 parent 重赋值");
      let existing = resolve_scope(parent)
        .expect("parent 句柄出自 register_scope，注册表内恒可解析（契约 1）")
        .type_alias_type_pack_parameters
        .get(&n)
        .copied();
      let cached = match existing {
        Some(c) if !c.is_null() => c,
        _ => {
          let mut gtp = GenericTypePack {
            index: 0,
            level,
            // arena 字段既有约定：GenericTypePack.scope 为可空 `*mut Scope` 句柄槽
            // （结构层契约），ctor 空值按该约定保留，不属本批次逻辑层哨兵。
            scope: null_mut(),
            name: n.clone(),
            explicit_name: false,
            polarity: Polarity::Unknown,
          };
          gtp.generic_type_pack_type_level_name(level, &n);
          let new_tp = self.add_type_pack_type_pack_var(TypePackVar::from(gtp));
          // 同类型参数分支——写入与 C++
          // `parentScope->typeAliasTypePackParameters[name]` 同位，检查期单线程独占。
          resolve_scope_mut(parent)
            .expect("parent 句柄出自 register_scope，注册表内恒可解析（契约 1）")
            .type_alias_type_pack_parameters
            .insert(n.clone(), new_tp);
          new_tp
        }
      };

      generic_packs.push(GenericTypePackDefinition {
        tp: cached,
        default_value,
      });
      // C++ `scope->privateTypePackBindings` 的原地修改。
      alias(scope_ptr)
        .private_type_pack_bindings
        .insert(n, cached);
    }

    GenericTypeDefinitions {
      generic_types: generics,
      generic_packs,
    }
  }
}
