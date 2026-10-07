use ulua_common::{fflag::DebugLuauUserDefinedClasses, records::variant::Variant2};

use crate::{
  records::{
    ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
    ast_stat_class::AstStatClass, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref, ast_type_visit_ref},
};

impl_visitable!(AstStatClass, StatClass, |this, visitor| {
  ulua_common::LUAU_ASSERT!(DebugLuauUserDefinedClasses.get());

  // super_ 为 null 或 parser 写入的存活 arena 表达式指针（可选字段语义）：
  // null 折叠与解引用经 `OptNode` 句柄边界（等价旧守卫），调用点无 unsafe。
  if let Some(super_) = OptNode::from_ptr(this.super_).get_mut() {
    ast_expr_visit_ref(super_, visitor);
  }

  for member in this.members.iter() {
    match member {
      Variant2::V0(prop) => {
        let prop: &AstClassProperty = prop;
        // ty 为 null 或存活 arena 类型指针，同上经句柄边界折叠跳过。
        if let Some(ty) = OptNode::from_ptr(prop.ty).get_mut() {
          ast_type_visit_ref(ty, visitor);
        }
      }
      Variant2::V1(method) => {
        let method: &AstClassMethod = method;
        // parser 保证 class method 的 `function` 子槽非空且指向 arena 存活节点；
        // 静态类型已知，直接走 `AstVisitable::visit`（cpp 虚分发同一目标）。
        if let Some(function) = OptNode::from_ptr(method.function).get_mut() {
          function.visit(visitor);
        }
      }
    }
  }
});
