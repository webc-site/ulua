use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::records::{
  ast_array::AstArray, ast_generic_type_pack::AstGenericTypePack, node_handle::Nodes,
};

use crate::{
  enums::polarity::Polarity,
  functions::arc_as_mut::arc_as_mut,
  records::{
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
    // iter_nodes：unsafe 收口在 AstArray::iter_nodes（元素由 parser 成对写入，
    // 恒指向存活节点）。
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
    let scope_ptr = arc_as_mut(scope);

    for generic in generics {
      // 元素契约：parser 成对写入 data/size 的数组，元素是声明处非空的
      // AstGenericTypePack 槽（指向 parse arena 存活节点），只读 name。
      let generic_name_str = generic.name.as_str_or_empty().to_string();

      let mut generic_ty: Option<TypePackId> = None;

      if use_cache
        && let Some(parent_scope) = scope.parent.and_then(resolve_scope)
        && let Some(type_pack_id) = parent_scope
          .type_alias_type_pack_parameters
          .get(&generic_name_str)
      {
        generic_ty = Some(*type_pack_id);
      }

      if generic_ty.is_none() {
        let mut gtp = GenericTypePack {
          index: 0,
          level: Default::default(),
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
        generic_ty = Some(
          // Safety: self.arena.as_ptr() 在 ConstraintGenerator 构造时接线为非空裸指针，指向
          // 本次 check 会话存活的类型 arena（bump 块地址不移动），add 只追加节点。
          { self.arena.get_mut().add_type_pack_t(gtp) },
        );

        if let Some(parent_scope) = scope.parent {
          // 缓存写回经 resolve_scope_mut（scope_registry 写回纪律，替换原借
          // parent Arc 的 arc_as_mut 裸写）：单线程串行，写入时刻无其他存活借用，
          // 与 C++ 直接改父作用域 map 同语义。
          resolve_scope_mut(parent_scope)
            .expect("parent 句柄出自 register_scope，注册表内恒可解析（契约 1）")
            .type_alias_type_pack_parameters
            .insert(
              generic_name_str.clone(),
              // Safety: generic_ty 在 is_none 分支内刚被赋 Some（或既有命中路径），至此恒 Some。
              generic_ty.expect("is_none 分支已即时赋 Some，至此恒为 Some"),
            );
        }
      }

      let default_ty: Option<TypePackId> = if generic.default_value.is_none() {
        None
      } else {
        let mut btp = BlockedTypePack {
          index: 0,
          owner: null_mut(),
        };
        btp.blocked_type_pack_blocked_type_pack();
        // Safety: self.arena.as_ptr() 非空接线且比本次解析长寿（同上），仅追加节点。
        Some({ self.arena.get_mut().add_type_pack_t(btp) })
      };

      if add_types {
        unsafe {
          // Safety: scope_ptr 由入口 arc_as_mut(scope)（&ScopePtr 借用的 Arc<Scope>）
          // 派生，Arc 存活至函数末；单线程内此刻无其他借用指向该 Scope，绑定表写入
          // 与 C++ 原实现同址同值，无别名冲突。
          (*scope_ptr).private_type_pack_bindings.insert(
            generic_name_str.clone(),
            // Safety: 同上位图写入点——两条路径均保证 generic_ty 已被赋 Some。
            generic_ty.expect("两条构造路径均以 Some 收尾，至此恒为 Some"),
          );
        }
      }

      result.push((
        generic_name_str,
        GenericTypePackDefinition {
          // Safety: 同上——generic_ty 恒由两路径之一赋 Some。
          tp: generic_ty.expect("两条构造路径均以 Some 收尾，至此恒为 Some"),
          default_value: default_ty,
        },
      ));
    }

    result
  }
}
