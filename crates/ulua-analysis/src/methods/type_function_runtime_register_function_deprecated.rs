//! Faithful port of
//! `std::optional<std::string> TypeFunctionRuntime::registerFunction_DEPRECATED(AstStatTypeFunction* function)`
//! (Analysis/src/TypeFunctionRuntime.cpp:58-142).
use alloc::{
  ffi::CString,
  string::{String, ToString},
  vec::Vec,
};
use core::{
  ffi,
  ffi::{CStr, c_void},
  ptr::null_mut,
};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_ast::{
  records::{
    allocator::Allocator, ast_array::AstArray, ast_expr::AstExpr, ast_name_table::AstNameTable,
    ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_return::AstStatReturn,
    ast_stat_type_function::AstStatTypeFunction, location::Location, parse_result::ParseResult,
  },
  type_aliases::cst_node_map::CstNodeMap,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::functions::format::format;
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
  records::{compile_error::CompileError, compile_options::CompileOptions},
};
use ulua_vm::{
  functions::{
    lua_gettable::lua_gettable, lua_l_sandboxthread::lua_l_sandboxthread,
    lua_newthread::lua_newthread, lua_pushvalue::lua_pushvalue, lua_resume::lua_resume,
    lua_setreadonly::lua_setreadonly, lua_settable::lua_settable, lua_xmove::lua_xmove,
    luau_load::luau_load,
  },
  macros::{
    LUA_PUSHLIGHTUSERDATA, lua_globalsindex::LUA_GLOBALSINDEX, lua_pop::lua_pop,
    lua_registryindex::LUA_REGISTRYINDEX,
  },
  records::lua_state,
};

use crate::{
  functions::check_result_for_error_deprecated::check_result_for_error_deprecated,
  records::{
    luau_temp_thread_popper::LuauTempThreadPopper, type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::lua_state::LuaState,
};
impl TypeFunctionRuntime {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn register_function_deprecated(
    &mut self,
    function: *mut AstStatTypeFunction,
  ) -> Option<String> {
    unsafe {
      // If evaluation is disabled, we do not generate additional error messages
      if !self.allow_evaluation {
        return None;
      }

      // Do not evaluate type functions with parse errors inside
      if (*function).has_errors {
        return None;
      }

      self.prepare_state();

      // lua_State* global = state.get();
      let global = self.state.0;
      let global_vm = global as *mut lua_state::LuaState;

      // Fetch to check if function is already registered
      // LUA_PUSHLIGHTUSERDATA(global, function); lua_gettable(global, LUA_REGISTRYINDEX);
      LUA_PUSHLIGHTUSERDATA::LUA_PUSHLIGHTUSERDATA(global, function as *mut c_void);
      lua_gettable(global_vm, LUA_REGISTRYINDEX);

      // if (!lua_isnil(global, -1)) { lua_pop(global, 1); return std::nullopt; }
      if !ulua_vm::lua_isnil!(global_vm, -1) {
        lua_pop(global_vm, 1);
        return None;
      }

      lua_pop(global_vm, 1);

      // AstName name = function->name;
      let name = (*function).name;
      let name_str = ast_name_to_string(name.value);

      // Construct ParseResult containing the type function
      // Allocator allocator; AstNameTable names(allocator);
      let mut allocator = Allocator::new();
      let mut names = AstNameTable::new(&mut allocator);

      // AstExpr* exprFunction = function->body;
      let mut expr_function: *mut AstExpr = (*function).body as *mut AstExpr;
      // AstArray<AstExpr*> exprReturns{&exprFunction, 1};
      let expr_returns: AstArray<*mut AstExpr> = AstArray {
        data: &mut expr_function as *mut *mut AstExpr,
        size: 1,
      };
      // AstStatReturn stmtReturn{Location{}, exprReturns};
      let mut stmt_return = AstStatReturn::new(Location::default(), expr_returns);
      // AstStat* stmtArray[] = {&stmtReturn};
      let mut stmt_array: [*mut AstStat; 1] =
        [&mut stmt_return as *mut AstStatReturn as *mut AstStat];
      // AstArray<AstStat*> stmts{stmtArray, 1};
      let stmts: AstArray<*mut AstStat> = AstArray {
        data: stmt_array.as_mut_ptr(),
        size: 1,
      };
      // AstStatBlock exec{Location{}, stmts};
      let mut exec = AstStatBlock::new(Location::default(), stmts, true);
      // ParseResult parseResult{&exec, 1, {}, {}, {}, CstNodeMap{nullptr}};
      let parse_result = ParseResult {
        root: &mut exec as *mut AstStatBlock,
        lines: 1,
        hotcomments: Vec::new(),
        errors: Vec::new(),
        comment_locations: Vec::new(),
        cst_node_map: CstNodeMap::new(null_mut()),
      };

      // BytecodeBuilder builder;
      let mut builder = BytecodeBuilder::default();
      let options = CompileOptions::default();

      // try { compileOrThrow(builder, parseResult, names); }
      // catch (CompileError& e) {
      //     return format("'%s' type function failed to compile with error message: %s", name.value, e.what());
      // }
      let compile_outcome = catch_unwind(AssertUnwindSafe(|| {
        compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
          &mut builder,
          &parse_result,
          &mut names,
          &options,
        );
      }));

      if let Err(payload) = compile_outcome {
        if let Some(e) = payload.downcast_ref::<CompileError>() {
          let what = ToString::to_string(e);
          return Some(format(format_args!(
            "'{}' type function failed to compile with error message: {}",
            name_str, what
          )));
        }
        // Non-CompileError panic: re-raise to preserve faithful unwinding.
        resume_unwind(payload);
      }

      // std::string bytecode = builder.getBytecode();
      let bytecode = builder.get_bytecode().clone();

      // Separate sandboxed thread for individual execution and private globals
      // lua_State* l = lua_newthread(global);
      let l_vm = lua_newthread(global_vm);
      let l = l_vm as *mut LuaState;
      // luau_temp_thread_popper popper(global);
      let mut popper = LuauTempThreadPopper::new(global);

      // luaL_sandboxthread(l);
      lua_l_sandboxthread(l_vm);

      // lua_pushvalue(l, LUA_GLOBALSINDEX); lua_setreadonly(l, -1, true); lua_pop(l, 1);
      lua_pushvalue(l_vm, LUA_GLOBALSINDEX);
      lua_setreadonly(l_vm, -1, 1);
      lua_pop(l_vm, 1);

      // Load bytecode into Luau state
      // if (auto error = checkResultForError_DEPRECATED(l, name.value, luau_load(...))) return error;
      let name_c = CString::new(name_str.as_bytes()).unwrap_or_default();
      let load_result = luau_load(
        l_vm,
        name_c.as_ptr(),
        bytecode.as_ptr() as *const ffi::c_char,
        bytecode.len(),
        0,
      );
      if let Some(error) = check_result_for_error_deprecated(l, &name_str, load_result) {
        popper.luau_temp_thread_popper();
        return Some(error);
      }

      // Execute the global function which should return our user-defined type function
      // if (auto error = checkResultForError_DEPRECATED(l, name.value, lua_resume(l, nullptr, 0))) return error;
      let resume_result = lua_resume(l_vm, null_mut(), 0);
      if let Some(error) = check_result_for_error_deprecated(l, &name_str, resume_result) {
        popper.luau_temp_thread_popper();
        return Some(error);
      }

      // if (!lua_isfunction(l, -1)) {
      //     lua_pop(l, 1);
      //     return format("Could not find '%s' type function in the global scope", name.value);
      // }
      if !ulua_vm::lua_isfunction!(l_vm, -1) {
        lua_pop(l_vm, 1);
        popper.luau_temp_thread_popper();
        return Some(format(format_args!(
          "Could not find '{}' type function in the global scope",
          name_str
        )));
      }

      // Store resulting function in the registry
      // LUA_PUSHLIGHTUSERDATA(global, function); lua_xmove(l, global, 1); lua_settable(global, LUA_REGISTRYINDEX);
      LUA_PUSHLIGHTUSERDATA::LUA_PUSHLIGHTUSERDATA(global, function as *mut c_void);
      lua_xmove(l_vm, global_vm, 1);
      lua_settable(global_vm, LUA_REGISTRYINDEX);

      popper.luau_temp_thread_popper();
      None
    }
  }
}

/// Helper: read an `AstName.value` (`*const c_char`) into an owned `String`.
unsafe fn ast_name_to_string(value: *const ffi::c_char) -> String {
  if value.is_null() {
    String::new()
  } else {
    unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() }
  }
}
