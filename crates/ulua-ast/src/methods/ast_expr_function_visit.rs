use core::{ffi::c_void, hint::black_box};

use crate::{
  records::{ast_expr_function::AstExprFunction, ast_visitor::AstVisitor},
  visit::{AstVisitable, ast_stat_visit, ast_type_pack_visit, ast_type_visit},
};

impl AstVisitable for AstExprFunction {
  fn visit<V: AstVisitor + ?Sized>(&self, visitor: &mut V) {
    // cpp `AstExprFunction::visit(AstVisitor*)` 的 this 是非 const 指针：
    // visit 型 observer（Analysis 的 TypeAttacher.attachTypes）按 cpp 语义会在
    // dispatch 内写穿 this（回填 returnAnnotation，unit-test 的
    // decorateWithTypes 依赖它）。而本 trait 的 `&self` 参数在 IR 里携带
    // readonly+noalias 属性，attach 经其派生指针写同一内存即构成优化器可见的
    // 别名 UB——release（LTO）据此删除该 store，症状是 release-only 丢失函数
    // 返回类型注解（dev 布局不同未触达）。black_box 切断派生指针与 `&self`
    // 属性的关联，使 store 存活；这是 6 例 unit-test release-only 失败的修复。
    let this = black_box(self as *const Self) as *mut Self;
    unsafe {
      if visitor.visit_expr_function(this as *mut c_void) {
        for arg_ptr in (*this).args.as_slice() {
          let arg = &*(*arg_ptr);
          if !arg.annotation.is_null() {
            ast_type_visit(arg.annotation, visitor);
          }
        }

        if !(*this).vararg_annotation.is_null() {
          ast_type_pack_visit((*this).vararg_annotation, visitor);
        }

        if !(*this).return_annotation.is_null() {
          ast_type_pack_visit((*this).return_annotation, visitor);
        }

        ast_stat_visit((*this).body as *mut _, visitor);
      }
    }
  }
}

pub fn ast_expr_function_visit<V: AstVisitor + ?Sized>(this: &AstExprFunction, visitor: &mut V) {
  this.visit(visitor);
}
