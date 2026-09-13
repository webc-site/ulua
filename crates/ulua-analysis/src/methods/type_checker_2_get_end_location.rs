use core::cmp;

use ulua_ast::records::{ast_expr_function::AstExprFunction, location::Location};

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `function` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
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
