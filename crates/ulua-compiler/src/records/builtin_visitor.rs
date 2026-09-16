use core::ffi::{CStr, c_void};

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_varargs::AstExprVarargs, ast_local::AstLocal,
    ast_name::AstName, ast_name_table::AstNameTable, ast_node::AstNode, ast_visitor::AstVisitor,
  },
  rtti::ast_node_is,
};
use ulua_common::{
  enums::luau_builtin_function::LuauBuiltinFunction, records::dense_hash_map::DenseHashMap,
};

use crate::{
  enums::global::Global,
  functions::{
    get_builtin::get_builtin, get_builtin_function_id::get_builtin_function_id,
    get_global_state::get_global_state,
  },
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
  /// 构造：按 `disabled_builtins`（"lib.member" / "global" 列表）预填禁用表
  pub fn new(
    result: &mut DenseHashMap<*mut AstExprCall, i32>,
    globals: &DenseHashMap<AstName, Global>,
    variables: &DenseHashMap<*mut AstLocal, Variable>,
    options: &CompileOptions,
    names: &AstNameTable,
  ) -> Self {
    let mut builtin_is_disabled = [false; 256];

    // 两个分支共享的收尾：member 名有效且仍为 Default 全局时，按 Builtin 反查
    // bfid 并置禁用位（object 侧由调用方保证合法：库分支要求非空，全局分支传空名）
    let mut disable_builtin = |object: AstName, member: AstName| {
      if !member.is_null() && get_global_state(globals, member) == Global::Default {
        let builtin = Builtin {
          object,
          method: member,
        };

        let bfid = get_builtin_function_id(&builtin, options);
        if bfid >= 0 && (bfid as usize) < 256 {
          builtin_is_disabled[bfid as usize] = true;
        }
      }
    };

    let disabled_builtins = options.disabled_builtins;
    if !disabled_builtins.is_null() {
      unsafe {
        let mut ptr = disabled_builtins;
        while !(*ptr).is_null() {
          let bytes = CStr::from_ptr(*ptr).to_bytes();

          if let Some(dot) = bytes.iter().position(|&b| b == b'.') {
            let library = names.get_slice(&bytes[..dot]);
            if !library.is_null() {
              let name = names.get_slice(&bytes[dot + 1..]);
              disable_builtin(library, name);
            }
          } else {
            let name = names.get_slice(bytes);
            disable_builtin(AstName::new(), name);
          }
          ptr = ptr.add(1);
        }
      }
    }

    Self {
      result: result as *mut _,
      builtin_is_disabled,
      globals: globals as *const _,
      variables: variables as *const _,
      options: options as *const _,
    }
  }
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
