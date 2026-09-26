use alloc::vec::Vec;

use ulua_ast::records::{
  ast_array::AstArray, ast_generic_type::AstGenericType, node_handle::Nodes,
};

use crate::{
  enums::polarity::Polarity,
  functions::arc_as_mut::arc_as_mut,
  records::{
    blocked_type::BlockedType,
    constraint_generator::ConstraintGenerator,
    generic_type::GenericType,
    generic_type_definition::GenericTypeDefinition,
    scope_registry::{resolve_scope, resolve_scope_mut},
    type_fun::TypeFun,
  },
  type_aliases::{name_type::Name, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  pub fn create_generics(
    &mut self,
    scope: &ScopePtr,
    generics: AstArray<*mut AstGenericType>,
    use_cache: bool,
    add_types: bool,
  ) -> Vec<(Name, GenericTypeDefinition)> {
    // iter_nodes：arena 写入的节点指针批量解引用为共享引用，unsafe 收口在
    // AstArray::iter_nodes 内部（元素由 parser 成对写入，恒指向存活节点）。
    self.create_generics_impl(scope, generics.iter_nodes(), use_cache, add_types)
  }

  /// `&Nodes` 槽位形态：`AstExprFunction{generics}` 句柄化后,
  /// `Node::get` 直出 arena 存活只读引用,与 [`Self::create_generics`] 同义。
  pub fn create_generics_nodes(
    &mut self,
    scope: &ScopePtr,
    generics: &Nodes<AstGenericType>,
    use_cache: bool,
    add_types: bool,
  ) -> Vec<(Name, GenericTypeDefinition)> {
    self.create_generics_impl(scope, generics.iter(), use_cache, add_types)
  }

  fn create_generics_impl<'g>(
    &mut self,
    scope: &ScopePtr,
    generics: impl Iterator<Item = &'g AstGenericType>,
    use_cache: bool,
    add_types: bool,
  ) -> Vec<(Name, GenericTypeDefinition)> {
    let mut result: Vec<(Name, GenericTypeDefinition)> = Vec::new();

    // 本 fn 仅以类型别名/函数定义 scope 调用，其父作用域恒存在
    // （cpp `scope->parent->` 直 deref 同前提，root scope 不入本路径）；父以
    // ScopeId 句柄横传：只读走 resolve_scope，缓存写回走 resolve_scope_mut
    // （scope_registry 写回纪律，替换原借 parent Arc 的 arc_as_mut 裸写）。
    let parent_scope = scope
      .parent
      .expect("定义类 scope 恒有父（cpp scope->parent 直 deref 同位）");
    let scope_ptr = arc_as_mut(scope);

    for generic in generics {
      // 元素契约：parser 分配进 AST arena、非空且存活至本次检查结束（无 Optional
      // 注解的子节点被 parser 保证非空）；此处仅只读 `name` 字段并拷出字符串。
      let generic_name = generic.name.as_str_or_empty().to_string();

      let generic_ty: TypeId;

      if use_cache {
        let cached = resolve_scope(parent_scope)
          .expect("parent 句柄出自 register_scope，注册表内恒可解析（契约 1）")
          .type_alias_type_parameters
          .get(generic_name.as_str())
          .copied();
        if let Some(ty) = cached {
          generic_ty = ty;
        } else {
          // Safety: `self.arena.as_ptr()` 是构造期接线的 `NotNull<TypeArena>` 裸指针字段，
          // 非空且比本 ConstraintGenerator 长寿；`add_type` 取 `&mut` 是对该 arena
          // 的独占短借用，语句结束即归还。bump arena 块地址永不移动，新类型驻留
          // 其中。传入的 `scope_ptr` 仅作身份句柄存入节点，本调用期间不解引用。
          generic_ty = {
            self
              .arena
              .get_mut()
              .add_type(GenericType::generic_type_scope_name_polarity(
                scope_ptr,
                generic_name.clone(),
                Polarity::None,
              ))
          };
          // 缓存写回：`resolve_scope_mut` 同一时刻至多一个存活 `&mut`
          // （scope_registry 写回纪律），与上一 arena 借用互不相交、不并存——
          // 语义等同 cpp 直接改父作用域 map。
          resolve_scope_mut(parent_scope)
            .expect("parent 句柄出自 register_scope，注册表内恒可解析（契约 1）")
            .type_alias_type_parameters
            .insert(generic_name.clone(), generic_ty);
        }
      } else {
        // Safety: 同缓存分支——`self.arena.as_ptr()` 非空且长寿，`add_type` 为独占短借用，
        // bump arena 块地址稳定；`scope_ptr` 仅作身份句柄存留。
        generic_ty = {
          self
            .arena
            .get_mut()
            .add_type(GenericType::generic_type_scope_name_polarity(
              scope_ptr,
              generic_name.clone(),
              Polarity::None,
            ))
        };
        // 缓存写回同上：resolve_scope_mut 独占可变借用，单线程串行、无并发别名。
        resolve_scope_mut(parent_scope)
          .expect("parent 句柄出自 register_scope，注册表内恒可解析（契约 1）")
          .type_alias_type_parameters
          .insert(generic_name.clone(), generic_ty);
      }

      let default_ty: Option<TypeId> = if generic.default_value.is_none() {
        None
      } else {
        // Safety: `self.arena.as_ptr()` 为非空长寿 arena，`add_type` 独占短借用，驻留
        // BlockedType 占位节点（对应 C++ `arena->addType(BlockedType{})`）。
        { self.arena.get_mut().add_type(BlockedType::default()) }.into()
      };

      if add_types {
        // Safety: `scope_ptr` 由 `arc_as_mut(scope)` 得到，指向形参 `scope` 这枚
        // 存活 `Arc<Scope>` 的主体；单线程串行、本块独占重建 `&mut` 写
        // privateTypeBindings，无并发别名（对应 C++ 直接写 `scope->privateTypeBindings`）。
        unsafe {
          (*scope_ptr)
            .private_type_bindings
            .insert(generic_name.clone(), TypeFun::type_fun_type_id(generic_ty));
        }
      }

      result.push((
        generic_name,
        GenericTypeDefinition {
          ty: generic_ty,
          default_value: default_ty,
        },
      ));
    }

    result
  }
}
