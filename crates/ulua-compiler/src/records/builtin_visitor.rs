use core::ffi::c_void;

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_varargs::AstExprVarargs, ast_local::AstLocal,
    ast_name::AstName, ast_node::AstNode, ast_visitor::AstVisitor,
  },
  rtti::ast_node_is,
};
use ulua_common::{
  enums::luau_builtin_function::LuauBuiltinFunction, records::dense_hash_map::DenseHashMap,
};

use crate::{
  enums::global::Global,
  functions::{get_builtin::get_builtin, get_builtin_function_id::get_builtin_function_id},
  records::{builtin::Builtin, compile_options::CompileOptions, variable::Variable},
};

#[derive(Debug, Clone)]
pub struct BuiltinVisitor {
  pub(crate) result: *mut DenseHashMap<*mut AstExprCall, i32>,
  pub(crate) builtin_is_disabled: [bool; 256],

  pub(crate) globals: *const DenseHashMap<AstName, Global>,
  pub(crate) variables: *const DenseHashMap<*mut AstLocal, Variable>,

  pub(crate) options: *const CompileOptions,
}

impl BuiltinVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn visit(&mut self, node: *mut AstExprCall) -> bool {
    unsafe {
      let options = &*self.options;
      let globals = &*self.globals;
      let variables = &*self.variables;
      let result = &mut *self.result;

      // C++ `getBuiltin(node->func, ...)` — pass the call's FUNCTION (the
      // `math.max` reference), not the whole call expression. The model
      // passed `node` (the call), so get_builtin never resolved a builtin
      // and nothing was registered for folding.
      let builtin = if (*node).self_ {
        Builtin::default()
      } else {
        get_builtin((*node).func, globals, variables)
      };

      if builtin.empty() {
        return true;
      }

      let mut bfid = get_builtin_function_id(&builtin, options);

      if bfid >= 0 && self.builtin_is_disabled[bfid as usize] {
        bfid = -1;
      }

      // getBuiltinFunctionId optimistically assumes all select() calls are builtin but actually
      // the second argument must be a vararg
      // C++: bfid == LBF_SELECT_VARARG && !(args.size == 2 && args.data[1]->is<AstExprVarargs>())
      if bfid == LuauBuiltinFunction::LBF_SELECT_VARARG as i32 {
        let is_select_arity_2 = (*node).args.len() == 2;
        let second_arg_is_vararg = is_select_arity_2 && {
          let arg1 = *(*node).args.data.add(1);
          ast_node_is::<AstExprVarargs>(arg1 as *mut AstNode)
        };

        if !(is_select_arity_2 && second_arg_is_vararg) {
          bfid = -1;
        }
      }

      if bfid >= 0 {
        // C++ `result[node] = bfid` overwrites.
        *result.get_or_insert(node) = bfid;
      }

      true
    }
  }
}

// C++ `BuiltinVisitor : AstVisitor` overrides `visit(AstExprCall*)`. The
// generated `visit(&mut self, *mut AstExprCall) -> bool` method existed but was
// NEVER wired to the visitor trait — so `analyzeBuiltins` (which must traverse
// the whole AST) registered nothing and ALL optimization-level-2 builtin
// constant-folding silently no-op'd. This impl dispatches every call node to it.
impl AstVisitor for BuiltinVisitor {
  fn visit_expr_call(&mut self, node: *mut c_void) -> bool {
    unsafe { self.visit(node as *mut AstExprCall) }
  }
}
