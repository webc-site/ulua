use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::records::{
  ast_array::AstArray, ast_generic_type_pack::AstGenericTypePack, node_handle::Nodes,
};

use crate::{
  enums::polarity::Polarity,
  functions::shared_mut::shared_mut,
  records::{
    arena_handle::alias,
    blocked_type_pack::BlockedTypePack,
    constraint_generator::ConstraintGenerator,
    generic_type_pack::GenericTypePack,
    generic_type_pack_definition::GenericTypePackDefinition,
    scope_registry::{resolve_scope, resolve_scope_mut},
  },
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};
impl ConstraintGenerator {
  pub fn create_generic_packs(
    &mut self,
    scope: &ScopePtr,
    generics: AstArray<*mut AstGenericTypePack>,
    use_cache: bool,
    add_types: bool,
  ) -> Vec<(Name, GenericTypePackDefinition)> {
    // iter_nodes：arena 解引用收口在 `AstArray::iter_nodes`（元素由 parser 成对
    // 写入 data/size，恒指向存活节点），本层只拿到共享引用。
    self.create_generic_packs_impl(scope, generics.iter_nodes(), use_cache, add_types)
  }

  /// `&Nodes` 槽位形态：`AstExprFunction{genericPacks}` 句柄化后,
  /// `Node::get` 直出 arena 存活只读引用,与 [`Self::create_generic_packs`] 同义。
  pub fn create_generic_packs_nodes(
    &mut self,
    scope: &ScopePtr,
    generics: &Nodes<AstGenericTypePack>,
    use_cache: bool,
    add_types: bool,
  ) -> Vec<(Name, GenericTypePackDefinition)> {
    self.create_generic_packs_impl(scope, generics.iter(), use_cache, add_types)
  }

  fn create_generic_packs_impl<'g>(
    &mut self,
    scope: &ScopePtr,
    generics: impl Iterator<Item = &'g AstGenericTypePack>,
    use_cache: bool,
    add_types: bool,
  ) -> Vec<(Name, GenericTypePackDefinition)> {
    let mut result: Vec<(Name, GenericTypePackDefinition)> = Vec::new();
    let scope_ptr = shared_mut(scope);

    for generic in generics {
      // 元素契约：parser 成对写入 data/size 的数组，元素是声明处非空的
      // AstGenericTypePack 槽（指向 parse arena 存活节点），只读 name。
      let generic_name_str = generic.name.as_str_or_empty().to_string();

      // 缓存命中形态（cpp `useCache` + 父作用域同名 genericPack 查询）直接复用既有
      // TypePackId；未命中才新建。以 `Option` 判定取代可空哨兵 + 事后 expect。
      let cached_ty = scope
        .parent
        .and_then(resolve_scope)
        .filter(|_| use_cache)
        .and_then(|parent| {
          parent
            .type_alias_type_pack_parameters
            .get(&generic_name_str)
            .copied()
        });

      let generic_ty: TypePackId = match cached_ty {
        Some(tp) => tp,
        None => {
          let mut gtp = GenericTypePack {
            index: 0,
            level: Default::default(),
            // `GenericTypePack::scope` 是记录内的裸 Scope 句柄字段，紧接着由
            // `generic_type_pack_scope_name_polarity` 写入真实 scope（cpp 构造器同序）：
            // 该零值是记录布局的初值形态，非逻辑层可空哨兵。
            scope: null_mut(),
            name: Default::default(),
            explicit_name: false,
            polarity: Polarity::None,
          };
          gtp.generic_type_pack_scope_name_polarity(
            scope_ptr,
            generic_name_str.clone(),
            Polarity::None,
          );
          // Safety: `self.arena` 是构造期接线的非空 `Handle<TypeArena>`，指向本次 check
          // 会话存活的类型 arena（bump 块地址不移动），`get_mut` 仅追加节点。
          let fresh = self.arena.get_mut().add_type_pack_t(gtp);

          if let Some(parent_scope) = scope.parent {
            // 缓存写回经 resolve_scope_mut（scope_registry 写回纪律，替换原借
            // parent Arc 的 shared_mut 裸写）：单线程串行，写入时刻无其他存活借用，
            // 与 C++ 直接改父作用域 map 同语义。
            resolve_scope_mut(parent_scope)
              .expect("parent 句柄出自 register_scope，注册表内恒可解析（契约 1）")
              .type_alias_type_pack_parameters
              .insert(generic_name_str.clone(), fresh);
          }

          fresh
        }
      };

      let default_ty: Option<TypePackId> = if generic.default_value.is_some() {
        // Safety: 同上——arena 句柄非空长寿，仅追加 BlockedTypePack 节点。
        Some(self.arena.get_mut().add_type_pack_t(BlockedTypePack::new()))
      } else {
        None
      };

      if add_types {
        // scope_ptr 由入口 shared_mut(scope)（&ScopePtr 借用的 Arc<Scope>）
        // 派生，Arc 存活至函数末；绑定表写入经 alias 门面即时物化（单线程
        // 独占写），与 C++ 原实现同址同值。
        alias(scope_ptr)
          .private_type_pack_bindings
          .insert(generic_name_str.clone(), generic_ty);
      }

      result.push((
        generic_name_str,
        GenericTypePackDefinition {
          tp: generic_ty,
          default_value: default_ty,
        },
      ));
    }

    result
  }
}
