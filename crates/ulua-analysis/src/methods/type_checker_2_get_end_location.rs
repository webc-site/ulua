use core::cmp;

use ulua_ast::records::{ast_expr_function::AstExprFunction, location::Location};

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `function` 非空、对齐，指向 parse arena 中存活至本次 check 结束、地址稳定的
  /// `AstExprFunction`；本函数仅只读其 `location` 字段。cpp `Analysis/src/TypeChecker2.cpp:346`
  /// （`Location TypeChecker2::getEndLocation(const AstExprFunction*)`）。单线程。
  pub unsafe fn get_end_location(&self, function: *const AstExprFunction) -> Location {
    let function = unsafe { &*function };
    let mut loc = function.base.base.location;

    if loc.begin.line != loc.end.line {
      let mut begin = loc.end;
      begin.column = cmp::max(0, begin.column as i32 - 3) as u32;
      loc = Location::new(begin, loc.end);
    }

    loc
  }
}
