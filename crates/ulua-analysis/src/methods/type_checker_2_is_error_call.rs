use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_global::AstExprGlobal,
  },
  rtti::ast_node_try_as_ptr,
};

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `call` 非空、对齐，指向 parse 树中存活至本次 check 结束、地址稳定的 `AstExprCall`
  /// （BuiltinAllocator bump 块不移动）。函数沿 RTTI 只读遍历 `func`/`args` 子节点，无并发可变别名。
  /// cpp `Analysis/src/TypeChecker2.cpp:359`（`bool TypeChecker2::isErrorCall(const AstExprCall*)`）。单线程。
  // cpp TypeChecker2.cpp:359
  pub unsafe fn is_error_call(&mut self, call: *const AstExprCall) -> bool {
    // Safety: `call` 由 TypeChecker2 的 AST 访问器传入（parse 树的 AstExprCall 节点，
    // 树根在 SourceModule.allocator 持有的 BuiltinAllocator 堆块上，bump 块地址
    // 不移动），在整次 check 期间存活且非空（本函数契约）；只读共享借用重建无别名。
    let call = unsafe { &*call };

    // Safety: parser 保证 AstExprCall.func 子节点指针非空（非 Optional 字段）；
    // ast_node_try_as_ptr 按 RTTI class index 分派——未命中返回 None，命中即动态
    // 类型确为 AstExprGlobal，且 repr(C) 单继承首字段基址重合使 cast 指向完整节点；
    // 节点随 parse 树存活至 check 结束。
    let Some(global) = (unsafe { ast_node_try_as_ptr::<AstExprGlobal>(call.func) }) else {
      return false;
    };
    let name = global.name.as_str_or_empty();

    if name == "error" {
      return true;
    }
    if name == "assert" {
      return call.args.is_empty()
        || call.args.first().is_some_and(|&arg| unsafe {
          ast_node_try_as_ptr::<AstExprConstantBool>(arg).is_some_and(|b| !b.value)
        });
    }

    false
  }
}
