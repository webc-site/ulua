use alloc::vec::Vec;
use core::ptr::{NonNull, null_mut};

use ulua_ast::records::{
  ast_array::AstArray, ast_generic_type::AstGenericType, ast_generic_type_pack::AstGenericTypePack,
  ast_node::AstNode, node_handle::Nodes,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  records::{
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
      let mut default_value = None;

      if generic.default_value.is_some() {
        // Safety: default_value 是 parser 写入 AstGenericType 的缺省类型注解
        // 节点指针，指向上层 SourceModule 持有的 AST arena 中存活节点，且
        // 上方判 Some 已排除空；resolve_type 只读该节点，借用不超出调用。
        default_value = Some(self.resolve_type(scope.clone(), unsafe {
          &*generic.default_value.map_or(null_mut(), NonNull::as_ptr)
        }));
      }

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
      // Safety: scope_ptr 由入参 `&ScopePtr`（Arc<Scope>）导出，Arc 在函数
      // 全程存活；本行是纯 `&mut`-via-裸指针写，此前的共享读借入均已释放，
      // 与 C++ `scope->privateTypeBindings[n] = ...` 同位、同独占前提。
      unsafe {
        (*scope_ptr)
          .private_type_bindings
          .insert(n, TypeFun::type_fun_type_id(g));
      }
    }

    let mut generic_packs = Vec::new();

    for generic_pack in generic_pack_names {
      let mut default_value = None;

      if generic_pack.default_value.is_some() {
        // Safety: default_value 为 parser 写入的缺省类型包注解节点，指向
        // 存活 AST arena 节点且已判空；resolve 仅读，不产生并存可变借用。
        default_value = Some(self.resolve_type_pack_scope_ptr_ast_type_pack(
          scope.clone(),
          unsafe {
            &*generic_pack
              .default_value
              .map_or(null_mut(), NonNull::as_ptr)
          },
        ));
      }

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
      // Safety: scope_ptr 的 Arc<Scope> 全程存活且此刻无并存 Rust 借用，
      // 写 private_type_pack_bindings 对应 C++ `scope->privateTypePackBindings`
      // 的原地修改。
      unsafe {
        (*scope_ptr).private_type_pack_bindings.insert(n, cached);
      }
    }

    GenericTypeDefinitions {
      generic_types: generics,
      generic_packs,
    }
  }
}
