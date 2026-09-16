use core::ptr::null_mut;

use ulua_ast::records::{
  ast_expr_function::AstExprFunction, ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
  ast_type_pack_explicit::AstTypePackExplicit, location::Location,
};

use crate::{
  functions::flatten_type_pack::flatten_type_pack_id,
  records::{
    type_attacher::TypeAttacher, type_rehydration_options::TypeRehydrationOptions,
    type_rehydration_visitor::TypeRehydrationVisitor,
  },
  type_aliases::{synthetic_names::SyntheticNames, type_pack_id::TypePackId},
};
impl TypeAttacher {
  /// # Safety
  /// 调用方须保证 `fn_` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  ///
  /// 实现注记：本函数按 cpp `TypeAttacher::visit(AstExprFunction* fn)` 语义
  /// **写穿 `fn_`**（回填 returnAnnotation，Ast/src/Parser.cpp 侧字段为此可变）。
  /// 因此体内禁止构造 `&AstExprFunction` 共享引用（如曾经的
  /// `let fn_ref = unsafe { &*fn_ }`）——共享引用活跃期经裸指针写同一内存是
  /// 别名 UB；nightly/edition2024 下 rustc 会为局部共享引用发 readonly 别名
  /// 标注，release（LTO）据此丢弃该 store，症状为 decorateWithTypes 丢失
  /// 函数返回类型注解（unit-test 6 例 release-only 失败的根因）。字段读取
  /// 一律走 `(*fn_)` 裸指针。
  pub(crate) unsafe fn visit_ast_expr_function(&mut self, fn_: *mut AstExprFunction) -> bool {
    for &arg in unsafe { (*fn_).args.as_slice() } {
      unsafe { self.visit_local(arg) };
    }

    if unsafe { (*fn_).return_annotation.is_null() } {
      // C++ `if (auto result = getScope(fn->body->location))` — Rust
      // `get_scope` always resolves an enclosing scope.
      let body_location = unsafe { (*(*fn_).body).base.base.location };
      let result = self.get_scope(&body_location);
      let ret: TypePackId = result.return_type;
      let (_v, tail) = flatten_type_pack_id(ret);

      let mut variadic_annotation: *mut AstTypePack = null_mut();
      if let Some(tail_tp) = tail {
        let mut rehydrator =
          TypeRehydrationVisitor::type_rehydration_visitor_type_rehydration_visitor(
            self.allocator,
            &mut self.synthetic_names as *mut SyntheticNames,
            &TypeRehydrationOptions::default(),
          );
        variadic_annotation = rehydrator.rehydrate(tail_tp);
      }

      let types = self.type_ast_pack(ret);
      let type_list = AstTypeList {
        types,
        tail_type: variadic_annotation,
      };
      let allocator = unsafe { &mut *self.allocator };
      unsafe {
        (*fn_).return_annotation = allocator
          .alloc(AstTypePackExplicit::new(Location::default(), type_list))
          as *mut AstTypePack;
      }
    }

    true
  }
}
