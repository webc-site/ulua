use ulua_common::{fflag::DebugLuauUserDefinedClasses, records::variant::Variant2};

use crate::{
  records::{
    ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
    ast_stat_class::AstStatClass, ast_visitor::AstVisitor,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit, ast_type_visit},
};

impl_visitable!(AstStatClass, StatClass, |this, visitor| {
  ulua_common::LUAU_ASSERT!(DebugLuauUserDefinedClasses.get());

  // Safety: `super_` 为 null 或 parser 写入的存活 arena 表达式指针（可选字段语义）；
  // `ast_expr_visit` 对 null 内部短路（等价旧守卫），非空才只读遍历子节点。
  unsafe {
    ast_expr_visit(this.super_, visitor);
  }

  for member in this.members.iter() {
    match member {
      Variant2::V0(prop) => {
        let prop: &AstClassProperty = prop;
        // Safety: `ty` 为 null 或 parser 写入的存活 arena 类型指针；`ast_type_visit`
        // 对 null 内部短路（等价旧守卫），非空才只读遍历子节点。
        unsafe {
          ast_type_visit(prop.ty, visitor);
        }
      }
      Variant2::V1(method) => {
        let method: &AstClassMethod = method;
        // Safety: parser 保证 class method 的 `function` 子指针非空且指向 arena 存活节点；
        // `.cast()` 仅改变节点类型位（repr(C) 基址重合），`ast_expr_visit` 只读遍历，无 `&mut`。
        unsafe {
          ast_expr_visit(method.function.cast(), visitor);
        }
      }
    }
  }
});
